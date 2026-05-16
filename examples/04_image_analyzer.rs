use std::path::PathBuf;

use visionkit::prelude::*;

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = ImageAnalyzerConfiguration::new(
        ImageAnalysisTypes::TEXT | ImageAnalysisTypes::MACHINE_READABLE_CODE,
    )
    .with_locales(["en-US"]);
    println!(
        "supported OCR languages: {}",
        ImageAnalyzer::supported_text_recognition_languages()?.len()
    );

    if !ImageAnalyzer::is_supported() {
        println!("ImageAnalyzer is not supported on this Mac");
        return Ok(());
    }

    let analyzer = ImageAnalyzer::new()?;
    let asset_path = asset_path();
    let analyses = [
        (
            "imageAt:url",
            analyzer.analyze_image_at_path(&asset_path, ImageOrientation::Up, &configuration)?,
        ),
        (
            "nsimage",
            analyzer.analyze_ns_image_at_path(&asset_path, ImageOrientation::Up, &configuration)?,
        ),
        (
            "cgimage",
            analyzer.analyze_cg_image_at_path(&asset_path, ImageOrientation::Up, &configuration)?,
        ),
        (
            "ciimage",
            analyzer.analyze_ci_image_at_path(&asset_path, ImageOrientation::Up, &configuration)?,
        ),
        (
            "pixelBuffer",
            analyzer.analyze_pixel_buffer_at_path(
                &asset_path,
                ImageOrientation::Up,
                &configuration,
            )?,
        ),
    ];

    for (label, analysis) in analyses {
        println!("{label}: {}", analysis.transcript()?);
    }
    Ok(())
}
