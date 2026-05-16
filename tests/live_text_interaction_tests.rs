use std::hint::black_box;
use std::path::PathBuf;

use visionkit::prelude::*;

#[test]
fn live_text_interaction_round_trips_basic_state() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("examples")
            .join("05_live_text_interaction"),
    )
    .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    if stdout.contains("not supported on this mac") {
        return Ok(());
    }

    assert!(stdout.contains("contents rect:"));
    assert!(stdout.contains("delegate events:"));
    assert!(stdout.contains("tracking image size:"));
    assert!(stdout.contains("copy image tag:"));
    assert!(stdout.contains("selected ranges:"));
    assert!(stdout.contains("subject unavailable case:"));
    assert!(stdout.contains("live text button visible:"));
    Ok(())
}

#[test]
fn live_text_interaction_extended_types_are_exported() {
    let _ = std::mem::size_of::<LiveTextInteractionDelegate>();
    let _ = std::mem::size_of::<LiveTextContentView>();
    let _ = std::mem::size_of::<LiveTextTrackingImageView>();
    let _ = std::mem::size_of::<LiveTextSubject>();
    let _ = std::mem::size_of::<LiveTextImageData>();
    let _ = LiveTextSubjectUnavailable::ImageUnavailable;
    let _ = LiveTextTextRange::new(0, 0);
    let _ = LiveTextMenuTag::new(0);
    black_box(LiveTextInteraction::subject_at_point);
    let image_for_subjects_fn: fn(&LiveTextInteraction, &[LiveTextSubject]) -> Result<
        LiveTextImageData,
        VisionKitError,
    > = LiveTextInteraction::image_for_subjects;
    black_box(image_for_subjects_fn);
}
