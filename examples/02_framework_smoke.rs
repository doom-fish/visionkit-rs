use std::path::PathBuf;

use visionkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== VisionKit.framework smoke ==");
    println!("ImageAnalyzer supported: {}", ImageAnalyzer::is_supported());

    let languages = ImageAnalyzer::supported_text_recognition_languages()?;
    println!("supported OCR languages: {}", languages.len());
    println!(
        "sample OCR languages: {:?}",
        languages.iter().take(10).collect::<Vec<_>>()
    );

    if !ImageAnalyzer::is_supported() {
        println!("ImageAnalyzer is not supported on this Mac");
        return Ok(());
    }

    let analyzer = ImageAnalyzer::new()?;
    let asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png");
    let configuration =
        ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT).with_locales(["en-US"]);
    let analysis = analyzer.analyze_image_at_path(
        &asset_path,
        ImageOrientation::Up,
        &configuration,
    )?;

    println!(
        "has text results: {}",
        analysis.has_results(ImageAnalysisTypes::TEXT)?
    );
    println!("transcript: {:?}", analysis.transcript()?);
    Ok(())
}
