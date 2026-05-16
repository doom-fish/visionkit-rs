use std::path::PathBuf;

use visionkit::prelude::*;

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== VisionKit.framework smoke ==");
    println!("ImageAnalyzer supported: {}", ImageAnalyzer::is_supported());
    println!(
        "document camera available on macOS: {}",
        VNDocumentCameraViewController::is_available_on_current_platform()?
    );
    println!(
        "data scanner available on macOS: {}",
        DataScannerViewController::is_available_on_current_platform()?
    );

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
    let configuration = ImageAnalyzerConfiguration::new(
        ImageAnalysisTypes::TEXT | ImageAnalysisTypes::MACHINE_READABLE_CODE,
    )
    .with_locales(["en-US"]);
    let asset_path = asset_path();
    let analysis =
        analyzer.analyze_image_at_path(&asset_path, ImageOrientation::Up, &configuration)?;

    println!(
        "has text results: {}",
        analysis.has_results(ImageAnalysisTypes::TEXT)?
    );
    let transcript = analysis.transcript()?;
    println!("transcript: {transcript:?}");

    let interaction = LiveTextInteraction::new()?;
    interaction.track_image_at_path(&asset_path)?;
    interaction.set_analysis(&analysis)?;
    println!(
        "overlay preferred types: {}",
        interaction.preferred_interaction_types()?.bits()
    );
    let overlay_text = interaction.text();
    println!("overlay text: {overlay_text:?}");
    Ok(())
}
