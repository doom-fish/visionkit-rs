use std::path::PathBuf;
use std::time::{Duration, Instant};

use visionkit::prelude::*;

fn configuration() -> ImageAnalyzerConfiguration {
    ImageAnalyzerConfiguration::new(
        ImageAnalysisTypes::TEXT | ImageAnalysisTypes::MACHINE_READABLE_CODE,
    )
    .with_locales(["en-US"])
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

#[test]
fn analysis_off_main_without_a_main_run_loop_finishes_promptly(
) -> Result<(), Box<dyn std::error::Error>> {
    assert_ne!(std::thread::current().name(), Some("main"));
    if !ImageAnalyzer::is_supported() {
        return Ok(());
    }
    let analyzer = ImageAnalyzer::new()?;
    let started = Instant::now();
    let result =
        analyzer.analyze_image_at_path(asset_path(), ImageOrientation::Up, &configuration());
    assert!(started.elapsed() < Duration::from_secs(30));
    match result {
        Ok(analysis) => assert!(analysis.has_results(ImageAnalysisTypes::TEXT)?),
        Err(VisionKitError::MainRunLoopNotRunning(message)) => {
            assert!(message.contains("main thread"));
        }
        Err(other) => panic!("expected an analysis or MainRunLoopNotRunning, got {other:?}"),
    }
    Ok(())
}

#[test]
fn analysis_of_a_missing_file_is_an_invalid_argument() -> Result<(), Box<dyn std::error::Error>> {
    if !ImageAnalyzer::is_supported() {
        return Ok(());
    }
    let result = ImageAnalyzer::new()?.analyze_image_at_path(
        "/nonexistent/visionkit-rs/missing.png",
        ImageOrientation::Up,
        &configuration(),
    );
    assert!(matches!(result, Err(VisionKitError::InvalidArgument(_))));
    Ok(())
}

#[test]
fn configuration_and_language_queries_work() -> Result<(), Box<dyn std::error::Error>> {
    let languages = ImageAnalyzer::supported_text_recognition_languages()?;
    assert!(languages.iter().any(|language| language == "en-US"));

    let configuration = configuration();
    assert!(configuration
        .analysis_types()
        .contains(ImageAnalysisTypes::TEXT));
    assert_eq!(configuration.locales(), &[String::from("en-US")]);
    assert_eq!(ImageOrientation::Up.raw_value(), 1);
    Ok(())
}

#[test]
fn all_path_loaders_produce_transcripts() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("examples")
            .join("04_image_analyzer"),
    )
    .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    if stdout.contains("not supported on this mac") {
        return Ok(());
    }

    for label in [
        "imageat:url",
        "nsimage",
        "cgimage",
        "ciimage",
        "pixelbuffer",
    ] {
        assert!(stdout.contains(label));
    }
    assert!(stdout.contains("visionkit") || stdout.contains("smoke"));
    Ok(())
}
