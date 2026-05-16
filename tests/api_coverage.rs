use std::path::PathBuf;

use visionkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let languages = ImageAnalyzer::supported_text_recognition_languages()?;
    assert!(!languages.is_empty());

    let configuration =
        ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT).with_locales(["en-US"]);
    assert_eq!(configuration.analysis_types(), ImageAnalysisTypes::TEXT);
    assert_eq!(configuration.locales(), ["en-US"]);
    assert_eq!(ImageOrientation::Up.raw_value(), 1);

    if !ImageAnalyzer::is_supported() {
        return Ok(());
    }

    let analyzer = ImageAnalyzer::new()?;
    let asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png");
    let analysis = analyzer.analyze_image_at_path(
        &asset_path,
        ImageOrientation::Up,
        &configuration,
    )?;
    assert!(analysis.has_results(ImageAnalysisTypes::TEXT)?);

    let transcript = analysis.transcript()?.to_lowercase();
    assert!(transcript.contains("visionkit") || transcript.contains("smoke"));
    Ok(())
}
