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
        println!("[skip] ImageAnalyzer is not supported on this Mac");
        return Ok(());
    }
    let analysis = ImageAnalyzer::new()?.analyze_image_at_path(
        asset_path(),
        ImageOrientation::Up,
        &ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT),
    )?;
    let interaction = LiveTextInteraction::new()?;
    interaction.track_image_at_path(asset_path())?;
    interaction.set_analysis(&analysis)?;
    let text = match interaction.text() {
        Ok(text) => text,
        Err(VisionKitError::UnavailableOnThisMacOS(_)) => {
            println!("[skip] overlay text needs macOS 14");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    assert!(!text.is_empty(), "the overlay has no text to select");

    let largest_swift_int = usize::MAX >> 1;
    for range in [
        LiveTextTextRange::new(1, largest_swift_int),
        LiveTextTextRange::new(largest_swift_int, largest_swift_int),
    ] {
        let result = interaction.set_selected_ranges(&[range]);
        assert!(
            matches!(result, Err(VisionKitError::InvalidArgument(_))),
            "expected InvalidArgument for {range:?}, got {result:?}"
        );
    }
    let result = interaction.set_selected_ranges(&[LiveTextTextRange::new(0, usize::MAX)]);
    assert!(result.is_err());
    interaction.set_selected_ranges(&[LiveTextTextRange::new(0, 1)])?;
    println!("overflowing selection ranges are rejected");
    Ok(())
}
