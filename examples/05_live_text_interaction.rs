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

    let asset_path = asset_path();
    let analyzer = ImageAnalyzer::new()?;
    let analysis = analyzer.analyze_image_at_path(
        &asset_path,
        ImageOrientation::Up,
        &ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT),
    )?;

    let interaction = LiveTextInteraction::new()?;
    interaction.track_image_at_path(&asset_path)?;
    interaction.set_analysis(&analysis)?;
    interaction.set_preferred_interaction_types(LiveTextInteractionTypes::AUTOMATIC_TEXT_ONLY)?;
    interaction.set_selectable_items_highlighted(true)?;
    let rect = interaction.contents_rect()?;
    println!("contents rect: {rect:?}");
    let overlay_text = interaction.text();
    println!("overlay text: {overlay_text:?}");
    println!(
        "live text button visible: {}",
        interaction.live_text_button_visible()?
    );
    println!(
        "supplementary hidden: {}",
        interaction.is_supplementary_interface_hidden()?
    );
    Ok(())
}
