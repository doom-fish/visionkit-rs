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

use std::ffi::{c_void, CStr};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use serde::Deserialize;

use crate::error::VisionKitError;
use crate::ffi;
use crate::image_analysis::ImageAnalysis;
use crate::image_analyzer::{ImageAnalyzerConfiguration, ImageOrientation};
use crate::live_text_interaction::LiveTextInteraction;
use crate::private::{json_cstring, path_to_cstring};

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

// ============================================================================
// AsyncImageAnalyzer
// ============================================================================

extern "C" fn analyze_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<ImageAnalysis>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        // result is a retained VKImageAnalysisBox pointer
        let analysis = ImageAnalysis::from_token(result.cast_mut());
        unsafe { AsyncCompletion::complete_ok(ctx, analysis) };
    } else {
        unsafe {
            AsyncCompletion::<ImageAnalysis>::complete_err(ctx, "Unknown error".into());
        };
    }
}

/// Future returned by [`AsyncImageAnalyzer::analyze_image_at_path`].
#[must_use = "futures do nothing unless polled"]
pub struct AnalyzeImageFuture {
    inner: AsyncCompletionFuture<ImageAnalysis>,
}

impl std::fmt::Debug for AnalyzeImageFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyzeImageFuture").finish_non_exhaustive()
    }
}

impl Future for AnalyzeImageFuture {
    type Output = Result<ImageAnalysis, VisionKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(VisionKitError::Unknown))
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
    /// This is a true async wrapper: the Swift bridge spawns a
    /// `Task { @MainActor … }`, calls
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

extern "C" fn subjects_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<String>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json = unsafe { cstring_result_to_string(result) };
        unsafe { AsyncCompletion::complete_ok(ctx, json) };
    } else {
        unsafe { AsyncCompletion::<String>::complete_err(ctx, "Unknown error".into()) };
    }
}

extern "C" fn subject_at_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<String>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json = unsafe { cstring_result_to_string(result) };
        unsafe { AsyncCompletion::complete_ok(ctx, json) };
    } else {
        unsafe { AsyncCompletion::<String>::complete_err(ctx, "Unknown error".into()) };
    }
}

/// Future returned by [`AsyncOverlaySubjects::subjects`].
#[must_use = "futures do nothing unless polled"]
pub struct SubjectsFuture {
    inner: AsyncCompletionFuture<String>,
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
            r.map_err(VisionKitError::Unknown).and_then(|json| {
                serde_json::from_str::<Vec<AnalysisSubjectBounds>>(&json).map_err(|e| {
                    VisionKitError::Unknown(format!(
                        "failed to decode subjects JSON from Swift bridge: {e}"
                    ))
                })
            })
        })
    }
}

/// Future returned by [`AsyncOverlaySubjects::subject_at`].
#[must_use = "futures do nothing unless polled"]
pub struct SubjectAtFuture {
    inner: AsyncCompletionFuture<String>,
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
            r.map_err(VisionKitError::Unknown).and_then(|json| {
                serde_json::from_str::<Option<AnalysisSubjectBounds>>(&json).map_err(|e| {
                    VisionKitError::Unknown(format!(
                        "failed to decode subject-at JSON from Swift bridge: {e}"
                    ))
                })
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
pub struct AsyncOverlaySubjects {
    // Raw token of the underlying VKLiveTextInteractionBox.
    // Caller must ensure the LiveTextInteraction outlives the returned futures.
    token: *mut c_void,
}

// SAFETY: the underlying VKLiveTextInteractionBox is ARC-counted and the callback
// fires exactly once. The callback carries a reference to the token across threads,
// and since the token is ARC-counted by Swift, it remains valid regardless of which
// thread completes the async operation. Sync is NOT implemented because the
// underlying ImageAnalysisOverlayView is a UI component that must be accessed from
// the main thread.
unsafe impl Send for AsyncOverlaySubjects {}

impl AsyncOverlaySubjects {
    /// Wrap a [`LiveTextInteraction`] for async subject queries.
    ///
    /// The `interaction` must remain alive for as long as the returned
    /// futures are pending.
    #[must_use]
    pub fn new(interaction: &LiveTextInteraction) -> Self {
        Self {
            token: interaction.raw_token(),
        }
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
                self.token,
                subjects_cb,
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
                self.token,
                x,
                y,
                subject_at_cb,
                ctx,
            );
        }
        SubjectAtFuture { inner: future }
    }
}

// ============================================================================
// block_on — run-loop-aware executor
// ============================================================================

struct NoopWake;
impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
    fn wake_by_ref(self: &Arc<Self>) {}
}

/// Drive a VisionKit async future to completion while pumping the Obj-C main
/// run loop between polls.
///
/// **Must be called from the main thread.** VisionKit Swift `Task { @MainActor
/// in }` thunks dispatch work to the main actor; without run-loop pumping the
/// calling thread would deadlock when the main run loop is not free (e.g.
/// inside `pollster::block_on`).
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
    let waker = Waker::from(Arc::new(NoopWake));
    let cx = &mut Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // Pump Obj-C RunLoop.main for 10 ms so Swift @MainActor Tasks
                // can make progress before the next poll.
                unsafe { ffi::image_analyzer::vk_pump_main_run_loop(10) };
            }
        }
    }
}
