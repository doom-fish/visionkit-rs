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
    let handle = std::mem::size_of::<*mut std::ffi::c_void>();
    assert_eq!(std::mem::size_of::<LiveTextInteractionDelegate>(), handle);
    assert_eq!(std::mem::size_of::<LiveTextContentView>(), handle);
    assert_eq!(std::mem::size_of::<LiveTextTrackingImageView>(), handle);
    assert_eq!(std::mem::size_of::<LiveTextSubject>(), handle);
    assert_eq!(
        LiveTextSubjectUnavailable::ImageUnavailable.to_string(),
        "subject image is unavailable"
    );
    let range = LiveTextTextRange::new(3, 4);
    assert_eq!((range.location, range.length, range.end()), (3, 4, 7));
    assert_eq!(LiveTextMenuTag::new(9).raw_value(), 9);
    black_box(LiveTextInteraction::subject_at_point);
    let image_for_subjects_fn: fn(&LiveTextInteraction, &[LiveTextSubject]) -> Result<
        LiveTextImageData,
        VisionKitError,
    > = LiveTextInteraction::image_for_subjects;
    black_box(image_for_subjects_fn);
}
