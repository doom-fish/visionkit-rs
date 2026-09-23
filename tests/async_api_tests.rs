#![cfg(feature = "async")]

/// Tests for `async_api` module.
///
/// Uses `visionkit::async_api::block_on` to drive futures synchronously.
/// Where `ImageAnalyzer` is not supported (CI / older hardware) tests are
/// skipped gracefully.
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

use visionkit::async_api::{block_on, AnalysisSubjectBounds, AsyncImageAnalyzer, AsyncOverlaySubjects};
use visionkit::{ImageAnalysisTypes, ImageAnalyzerConfiguration, ImageOrientation};
use visionkit::LiveTextInteraction;

struct PollCounter {
    polls: usize,
    ready_at: Instant,
    timer_started: bool,
}

impl Future for PollCounter {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<usize> {
        self.polls += 1;
        if Instant::now() >= self.ready_at {
            return Poll::Ready(self.polls);
        }
        if !self.timer_started {
            self.timer_started = true;
            let waker = cx.waker().clone();
            let ready_at = self.ready_at;
            thread::spawn(move || {
                thread::sleep(ready_at.saturating_duration_since(Instant::now()));
                waker.wake();
            });
        }
        Poll::Pending
    }
}

#[test]
fn test_block_on_parks_off_the_main_thread() {
    assert_ne!(thread::current().name(), Some("main"));
    let polls = block_on(PollCounter {
        polls: 0,
        ready_at: Instant::now() + Duration::from_millis(200),
        timer_started: false,
    });
    assert!(polls <= 3, "block_on polled {polls} times instead of parking");
}

fn skip_if_unsupported() -> bool {
    if !AsyncImageAnalyzer::is_supported() {
        eprintln!("[skip] ImageAnalyzer not supported on this Mac");
        return true;
    }
    false
}

// ============================================================================
// AsyncImageAnalyzer – happy path
// ============================================================================

#[test]
fn test_async_analyze_image_returns_analysis() {
    let binary = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("examples")
        .join("10_async_analyze");
    let output = std::process::Command::new(&binary)
        .output()
        .expect("run 10_async_analyze example");
    assert!(output.status.success(), "10_async_analyze exited with error:\n{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("not supported on this Mac") {
        return;
    }
    assert!(stdout.contains("has text:"), "expected 'has text:' in output:\n{stdout}");
    assert!(stdout.contains("transcript:"), "expected 'transcript:' in output:\n{stdout}");
}

// ============================================================================
// AsyncImageAnalyzer – error path (non-existent file)
// ============================================================================

#[test]
fn test_async_analyze_nonexistent_file_returns_error() {
    if skip_if_unsupported() {
        return;
    }
    block_on(async {
        let analyzer = AsyncImageAnalyzer::new().expect("create AsyncImageAnalyzer");
        let cfg = ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT);
        let result = analyzer
            .analyze_image_at_path(
                "/nonexistent/path/does_not_exist_12345.png",
                ImageOrientation::Up,
                &cfg,
            )
            .expect("create future")
            .await;
        assert!(result.is_err(), "expected error for missing file, got Ok");
    });
}

// ============================================================================
// AsyncOverlaySubjects.subjects – returns Vec (possibly empty)
// ============================================================================

/// Requires `ImageAnalysisOverlayView.subjects` to resolve on a view with a
/// loaded analysis attached to a live `NSWindow` — not available in headless tests.
/// Run with `cargo test -- --ignored` in a windowed context.
#[test]
#[ignore = "requires live NSWindow with loaded analysis"]
fn test_async_overlay_subjects_returns_vec() {
    let Ok(interaction) = LiveTextInteraction::new() else {
        eprintln!("[skip] LiveTextInteraction not available");
        return;
    };
    block_on(async {
        let overlay = AsyncOverlaySubjects::new(&interaction);
        let subjects = overlay.subjects().await.expect("subjects future");
        for s in &subjects {
            assert!(s.width >= 0.0, "subject width should be non-negative");
            assert!(s.height >= 0.0, "subject height should be non-negative");
        }
    });
}

// ============================================================================
// AsyncOverlaySubjects.subject_at – returns Option
// ============================================================================

/// Same headless-test limitation as `test_async_overlay_subjects_returns_vec`.
#[test]
#[ignore = "requires live NSWindow with loaded analysis"]
fn test_async_overlay_subject_at_returns_option() {
    let Ok(interaction) = LiveTextInteraction::new() else {
        eprintln!("[skip] LiveTextInteraction not available");
        return;
    };
    block_on(async {
        let overlay = AsyncOverlaySubjects::new(&interaction);
        let result = overlay.subject_at(0.0, 0.0).await;
        assert!(result.is_ok(), "subject_at should succeed");
        if let Ok(Some(s)) = result {
            assert!(s.width >= 0.0);
            assert!(s.height >= 0.0);
        }
    });
}

// ============================================================================
// AnalysisSubjectBounds – JSON round-trip (pure Rust, always runs)
// ============================================================================

#[test]
fn test_analysis_subject_bounds_json_round_trip() {
    let json = r#"{"x":10.5,"y":20.0,"width":100.0,"height":200.0}"#;
    let bounds: AnalysisSubjectBounds =
        serde_json::from_str(json).expect("deserialize bounds");
    assert!((bounds.x - 10.5).abs() < f64::EPSILON);
    assert!((bounds.y - 20.0).abs() < f64::EPSILON);
    assert!((bounds.width - 100.0).abs() < f64::EPSILON);
    assert!((bounds.height - 200.0).abs() < f64::EPSILON);
}

#[test]
fn test_analysis_subject_bounds_null_json() {
    let result: Option<AnalysisSubjectBounds> =
        serde_json::from_str("null").expect("deserialize null");
    assert!(result.is_none());
}

#[test]
fn test_analysis_subject_bounds_array_json() {
    let json = r#"[{"x":1.0,"y":2.0,"width":3.0,"height":4.0},{"x":5.0,"y":6.0,"width":7.0,"height":8.0}]"#;
    let subjects: Vec<AnalysisSubjectBounds> =
        serde_json::from_str(json).expect("deserialize subjects array");
    assert_eq!(subjects.len(), 2);
    assert!((subjects[0].x - 1.0).abs() < f64::EPSILON);
    assert!((subjects[1].height - 8.0).abs() < f64::EPSILON);
}
