//! Async API for `VisionKit`
//!
//! Enabled with the `async` Cargo feature. Every type here is an executor-agnostic
//! [`Future`] backed by a C callback fired from a Swift `Task { … }` thunk.
//!
//! ## Available types
//!
//! | Type | Wraps |
//! |---|---|
//! | [`AsyncImageAnalyzer`] | `ImageAnalyzer.analyze(imageAt:orientation:configuration:) async throws` |
//! | [`AsyncOverlaySubjects`] | `ImageAnalysisOverlayView.subjects async` and `.subject(at:) async` |
//!
//! ## Platform notes
//!
//! On macOS the subject APIs (`subjects`, `subject(at:)`) live on
//! `ImageAnalysisOverlayView` (Rust: [`LiveTextInteraction`]), not on
//! `ImageAnalysis` as on iOS. [`AsyncOverlaySubjects`] wraps this macOS surface.
//!
//! `DataScannerViewController` is an **iOS-only** API and is not available on
//! macOS. Its multi-delegate live-scan surface is a Tier-2 (Stream) concern in
//! any case. No wrappers are provided here.
//!
//! ## Design
//!
//! * Each async Swift API gets a `@_cdecl` thunk that accepts a C callback
//!   `(result, error, ctx)` and a `ctx` opaque pointer. The thunk spawns a
//!   Swift `Task`, awaits the Apple API, then fires the callback.
//! * The Rust side wraps that in a typed `Future` newtype backed by
//!   [`AsyncCompletionFuture<T>`](doom_fish_utils::completion::AsyncCompletionFuture)
//!   and maps the `String` error to [`VisionKitError`].
//! * Works with any async executor (`pollster`, Tokio, async-std, …).
//!
//! ## Example
//!
//! ```rust,no_run
//! use visionkit::async_api::AsyncImageAnalyzer;
//! use visionkit::{ImageAnalysisTypes, ImageAnalyzerConfiguration, ImageOrientation};
//!
//! # pollster::block_on(async {
//! let cfg = ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT);
//! let analysis = AsyncImageAnalyzer::new()?
//!     .analyze_image_at_path("/path/to/image.png", ImageOrientation::Up, &cfg)?
//!     .await?;
//! println!("transcript: {}", analysis.transcript()?);
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! # });
//! ```

use std::ffi::{c_char, c_int, c_void, CStr};
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

use doom_fish_utils::completion::{AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::panic_safe::catch_user_panic;
use serde::Deserialize;

use crate::error::VisionKitError;
use crate::ffi;
use crate::image_analysis::ImageAnalysis;
use crate::image_analyzer::{ImageAnalyzerConfiguration, ImageOrientation};
use crate::live_text_interaction::LiveTextInteraction;
use crate::private::{error_for_status, json_cstring, path_to_cstring};

// ============================================================================
// AnalysisSubjectBounds
// ============================================================================

/// Bounds rectangle for an `ImageAnalysisOverlayView.Subject` (macOS) or
/// `ImageAnalysisInteraction.Subject` (iOS).
///
/// The coordinate space matches what VisionKit reports for the analysis
/// resolution. `x` and `y` are the origin (top-left corner).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnalysisSubjectBounds {
    /// X coordinate of the origin.
    pub x: f64,
    /// Y coordinate of the origin.
    pub y: f64,
    /// Width of the bounding rectangle.
    pub width: f64,
    /// Height of the bounding rectangle.
    pub height: f64,
}

// ============================================================================
// Helpers
// ============================================================================

/// Copy a non-null C-string result pointer (reinterpreted as `*const i8`) to a `String`.
///
/// # Safety
/// `ptr` must be a valid, null-terminated C string that remains valid for the
/// duration of this call.  The callback contract guarantees this: the Swift
/// bridge passes a string from a `withCString` closure, so the pointer is live
/// for the duration of the (synchronous) callback invocation.
unsafe fn cstring_result_to_string(ptr: *const c_void) -> String {
    CStr::from_ptr(ptr.cast::<i8>())
        .to_str()
        .map_or_else(|_| String::new(), str::to_owned)
}

unsafe fn async_error(status: i32, error: *const c_char) -> VisionKitError {
    let message = if error.is_null() {
        format!("Swift bridge call failed with status code {status}")
    } else {
        CStr::from_ptr(error).to_string_lossy().into_owned()
    };
    error_for_status(status, message)
}

// ============================================================================
// AsyncImageAnalyzer
// ============================================================================

struct AnalysisToken(*mut c_void);

unsafe impl Send for AnalysisToken {}

impl AnalysisToken {
    fn into_analysis(self) -> ImageAnalysis {
        let token = ManuallyDrop::new(self);
        ImageAnalysis::from_token(token.0)
    }
}

impl Drop for AnalysisToken {
    fn drop(&mut self) {
        unsafe { ffi::image_analysis::vk_image_analysis_release(self.0) };
    }
}

type AnalysisOutcome = Result<AnalysisToken, VisionKitError>;

unsafe extern "C" fn analyze_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("visionkit::analyze_cb", move || {
        let outcome: AnalysisOutcome = if status != ffi::status::OK {
            Err(unsafe { async_error(status, error) })
        } else if result.is_null() {
            Err(VisionKitError::Unknown(
                "Swift bridge returned no image analysis".to_owned(),
            ))
        } else {
            Ok(AnalysisToken(result.cast_mut()))
        };
        unsafe { AsyncCompletion::complete_ok(ctx, outcome) };
    });
}

/// Future returned by [`AsyncImageAnalyzer::analyze_image_at_path`].
#[must_use = "futures do nothing unless polled"]
pub struct AnalyzeImageFuture {
    inner: AsyncCompletionFuture<AnalysisOutcome>,
}

impl std::fmt::Debug for AnalyzeImageFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyzeImageFuture").finish_non_exhaustive()
    }
}

impl Future for AnalyzeImageFuture {
    type Output = Result<ImageAnalysis, VisionKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|result| {
            result
                .map_err(VisionKitError::Unknown)
                .and_then(|outcome| outcome.map(AnalysisToken::into_analysis))
        })
    }
}

/// True-async entry point for `ImageAnalyzer.analyze(imageAt:orientation:configuration:)`.
///
/// Requires macOS 13+.  The returned [`AnalyzeImageFuture`] resolves to an
/// [`ImageAnalysis`] that can then be inspected synchronously via its existing
/// methods or have its subjects queried via [`AsyncOverlaySubjects`].
pub struct AsyncImageAnalyzer {
    token: *mut c_void,
}

// SAFETY: the underlying VKImageAnalyzerBox is reference-counted by Swift ARC
// and the callback fires exactly once. The callback carries a reference to the
// token across threads, and since the token is ARC-counted by Swift, it remains
// valid regardless of which thread completes the async operation. Sync is safe
// because VKImageAnalyzerBox is thread-safe (it's an immutable system framework
// class with no thread-local state).
unsafe impl Send for AsyncImageAnalyzer {}
unsafe impl Sync for AsyncImageAnalyzer {}

impl Drop for AsyncImageAnalyzer {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::image_analyzer::vk_image_analyzer_release(self.token) };
        }
    }
}

impl AsyncImageAnalyzer {
    /// Create a new `AsyncImageAnalyzer`.
    ///
    /// # Errors
    ///
    /// Returns [`VisionKitError::UnavailableOnThisMacOS`] if the device does
    /// not run macOS 13 or later.
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::image_analyzer::vk_image_analyzer_new() };
        if token.is_null() {
            return Err(VisionKitError::UnavailableOnThisMacOS(
                "ImageAnalyzer requires macOS 13+".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    /// Returns `true` when the current Mac supports `ImageAnalyzer`.
    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::image_analyzer::vk_image_analyzer_is_supported() != 0 }
    }

    /// Asynchronously analyze the image at `path` and return an [`ImageAnalysis`].
    ///
    /// This is a true async wrapper: the Swift bridge spawns a `Task`, calls
    /// `await analyzer.analyze(imageAt:orientation:configuration:)`, and fires
    /// a C callback when done.  The returned [`AnalyzeImageFuture`] resolves
    /// when that callback fires.
    ///
    /// # Errors
    ///
    /// Returns [`VisionKitError`] if the path cannot be represented as a C
    /// string, if the configuration cannot be serialized, or if VisionKit
    /// reports an analysis error.
    pub fn analyze_image_at_path<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        orientation: ImageOrientation,
        configuration: &ImageAnalyzerConfiguration,
    ) -> Result<AnalyzeImageFuture, VisionKitError> {
        let path_cs = path_to_cstring(path.as_ref())?;
        let cfg_cs = json_cstring(configuration)?;
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::image_analyzer::vk_image_analyzer_analyze_image_async(
                self.token,
                path_cs.as_ptr(),
                orientation.raw_value(),
                cfg_cs.as_ptr(),
                analyze_cb,
                ctx,
            );
        }
        Ok(AnalyzeImageFuture { inner: future })
    }
}

// ============================================================================
// AsyncOverlaySubjects
// ============================================================================

type JsonOutcome = Result<String, VisionKitError>;

unsafe extern "C" fn subject_json_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("visionkit::subject_json_cb", move || {
        let outcome: JsonOutcome = if status != ffi::status::OK {
            Err(unsafe { async_error(status, error) })
        } else if result.is_null() {
            Err(VisionKitError::Unknown(
                "Swift bridge returned no subject payload".to_owned(),
            ))
        } else {
            Ok(unsafe { cstring_result_to_string(result) })
        };
        unsafe { AsyncCompletion::complete_ok(ctx, outcome) };
    });
}

/// Future returned by [`AsyncOverlaySubjects::subjects`].
#[must_use = "futures do nothing unless polled"]
pub struct SubjectsFuture {
    inner: AsyncCompletionFuture<JsonOutcome>,
}

impl std::fmt::Debug for SubjectsFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubjectsFuture").finish_non_exhaustive()
    }
}

impl Future for SubjectsFuture {
    type Output = Result<Vec<AnalysisSubjectBounds>, VisionKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            let json = r.map_err(VisionKitError::Unknown)??;
            serde_json::from_str::<Vec<AnalysisSubjectBounds>>(&json).map_err(|e| {
                VisionKitError::Unknown(format!(
                    "failed to decode subjects JSON from Swift bridge: {e}"
                ))
            })
        })
    }
}

/// Future returned by [`AsyncOverlaySubjects::subject_at`].
#[must_use = "futures do nothing unless polled"]
pub struct SubjectAtFuture {
    inner: AsyncCompletionFuture<JsonOutcome>,
}

impl std::fmt::Debug for SubjectAtFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubjectAtFuture").finish_non_exhaustive()
    }
}

impl Future for SubjectAtFuture {
    type Output = Result<Option<AnalysisSubjectBounds>, VisionKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            let json = r.map_err(VisionKitError::Unknown)??;
            serde_json::from_str::<Option<AnalysisSubjectBounds>>(&json).map_err(|e| {
                VisionKitError::Unknown(format!(
                    "failed to decode subject-at JSON from Swift bridge: {e}"
                ))
            })
        })
    }
}

/// True-async entry point for `ImageAnalysisOverlayView.subjects` and
/// `ImageAnalysisOverlayView.subject(at:)` (macOS).
///
/// On macOS the subject APIs live on `ImageAnalysisOverlayView`
/// (Rust: [`LiveTextInteraction`]), not directly on `ImageAnalysis`.
/// Both require macOS 13+ and the [`crate::ImageAnalysisTypes::VISUAL_LOOK_UP`]
/// analysis type in the configuration used when the analysis was performed.
///
/// # Note on subject bounds coordinate space
///
/// Bounds are in the same coordinate space that VisionKit uses when populating
/// the overlay view.  That is, they are relative to the view's bounds as
/// configured via `setPreferredInteractionTypes` and `setAnalysis(_:)`.
pub struct AsyncOverlaySubjects<'a> {
    interaction: &'a LiveTextInteraction,
}

impl<'a> AsyncOverlaySubjects<'a> {
    /// Wrap a [`LiveTextInteraction`] for async subject queries.
    ///
    /// Each query takes its own reference to the overlay view before it
    /// returns, so pending futures stay valid if the `interaction` is dropped.
    #[must_use]
    pub const fn new(interaction: &'a LiveTextInteraction) -> Self {
        Self { interaction }
    }

    /// Asynchronously retrieve all subjects as their bounds rectangles.
    ///
    /// Internally calls `await overlayView.subjects` on the Swift side
    /// (requires `@MainActor`). Each subject's `bounds` is accessed
    /// synchronously within that actor context.
    ///
    /// Returns an empty `Vec` when no subjects are found or when the analysis
    /// did not include [`crate::ImageAnalysisTypes::VISUAL_LOOK_UP`].
    #[must_use = "returns a future that must be awaited"]
    pub fn subjects(&self) -> SubjectsFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::live_text_interaction::vk_live_text_overlay_subjects_async(
                self.interaction.raw_token(),
                subject_json_cb,
                ctx,
            );
        }
        SubjectsFuture { inner: future }
    }

    /// Asynchronously find the subject at the given overlay-coordinate point.
    ///
    /// Returns `Ok(None)` when no subject is present at `(x, y)`.
    #[must_use = "returns a future that must be awaited"]
    pub fn subject_at(&self, x: f64, y: f64) -> SubjectAtFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::live_text_interaction::vk_live_text_overlay_subject_at_async(
                self.interaction.raw_token(),
                x,
                y,
                subject_json_cb,
                ctx,
            );
        }
        SubjectAtFuture { inner: future }
    }
}

// ============================================================================
// block_on — run-loop-aware executor
// ============================================================================

struct ThreadWaker {
    thread: Thread,
    woken: AtomicBool,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.woken.store(true, Ordering::Release);
        self.thread.unpark();
    }
}

extern "C" {
    fn pthread_main_np() -> c_int;
}

/// Drive a VisionKit async future to completion on the current thread.
///
/// On the main thread it pumps the Obj-C main run loop until the future is
/// woken, because VisionKit completes its work through the main queue (and
/// the overlay-subject queries run on the main actor). On any other thread it
/// parks until woken; the futures then complete only while the main thread is
/// running its run loop, as it does in an AppKit app.
///
/// # Example
///
/// ```rust,no_run
/// use visionkit::async_api::{AsyncImageAnalyzer, block_on};
/// use visionkit::{ImageAnalysisTypes, ImageAnalyzerConfiguration, ImageOrientation};
///
/// let cfg = ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT);
/// let analysis = block_on(async {
///     AsyncImageAnalyzer::new()?
///         .analyze_image_at_path("/path/to/image.png", ImageOrientation::Up, &cfg)?
///         .await
/// });
/// ```
pub fn block_on<F: Future>(future: F) -> F::Output {
    let on_main_thread = unsafe { pthread_main_np() } != 0;
    let state = Arc::new(ThreadWaker {
        thread: thread::current(),
        woken: AtomicBool::new(false),
    });
    let waker = Waker::from(Arc::clone(&state));
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
        while !state.woken.swap(false, Ordering::Acquire) {
            if on_main_thread {
                unsafe { ffi::image_analyzer::vk_pump_main_run_loop(10) };
            } else {
                thread::park();
            }
        }
    }
}
