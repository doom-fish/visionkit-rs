use std::path::PathBuf;

use visionkit::prelude::*;

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !ImageAnalyzer::is_supported() {
        println!("ImageAnalyzer is not supported on this Mac");
        return Ok(());
    }

    let analyzer = ImageAnalyzer::new()?;
    let analysis = analyzer.analyze_cg_image_at_path(
        asset_path(),
        ImageOrientation::Up,
        &ImageAnalyzerConfiguration::new(
            ImageAnalysisTypes::TEXT | ImageAnalysisTypes::MACHINE_READABLE_CODE,
        ),
    )?;

    println!(
        "has text results: {}",
        analysis.has_results(ImageAnalysisTypes::TEXT)?
    );
    println!("transcript: {}", analysis.transcript()?);
    Ok(())
}
