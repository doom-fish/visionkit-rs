use std::path::PathBuf;

use visionkit::prelude::*;

fn configuration() -> ImageAnalyzerConfiguration {
    ImageAnalyzerConfiguration::new(
        ImageAnalysisTypes::TEXT | ImageAnalysisTypes::MACHINE_READABLE_CODE,
    )
    .with_locales(["en-US"])
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
