use std::hint::black_box;
use std::path::PathBuf;

use visionkit::prelude::*;

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn build_delegate() -> Result<LiveTextInteractionDelegate, Box<dyn std::error::Error>> {
    let delegate = LiveTextInteractionDelegate::new()?;
    delegate.set_should_begin(true)?;
    delegate.set_should_handle_key_down_event(true)?;
    delegate.set_should_show_menu_for_event(true)?;
    delegate.set_contents_rect_override(Some(Rect::default()))?;

    let content_view = LiveTextContentView::new()?;
    content_view.set_frame(Rect {
        x: 0.0,
        y: 0.0,
        width: 32.0,
        height: 32.0,
    })?;
    delegate.set_content_view(Some(&content_view))?;

    let updated_menu = LiveTextMenu {
        title: "VisionKit".to_owned(),
        items: vec![LiveTextMenuItem {
            title: "Copy".to_owned(),
            tag: LiveTextMenuTag::copy_image().map_or(0, LiveTextMenuTag::raw_value),
            is_separator: false,
            is_enabled: true,
            is_hidden: false,
            state: 0,
            submenu: None,
        }],
    };
    delegate.set_updated_menu(Some(&updated_menu))?;
    Ok(delegate)
}

fn print_selection_state(
    interaction: &LiveTextInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    match interaction.selected_ranges() {
        Ok(ranges) => {
            interaction.set_selected_ranges(&ranges)?;
            println!("selected ranges: {}", ranges.len());
        }
        Err(error) => println!("selected ranges: {error}"),
    }
    match interaction.selected_attributed_text() {
        Ok(text) => println!("selected attributed text runs: {}", text.runs.len()),
        Err(error) => println!("selected attributed text runs: {error}"),
    }
    match interaction.supplementary_interface_font() {
        Ok(font) => {
            interaction.set_supplementary_interface_font(font.as_ref())?;
            println!("supplementary font set: {}", font.is_some());
        }
        Err(error) => println!("supplementary font set: {error}"),
    }
    Ok(())
}

fn print_subject_state(
    interaction: &LiveTextInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "subject unavailable case: {:?}",
        LiveTextSubjectUnavailable::ImageUnavailable
    );
    match interaction.begin_subject_analysis_if_necessary() {
        Ok(()) => println!("subject analysis started"),
        Err(error) => println!("subject analysis started: {error}"),
    }
    match interaction.subjects() {
        Ok(subjects) => {
            println!("subjects: {}", subjects.len());
            println!(
                "highlighted subjects: {}",
                interaction.highlighted_subjects()?.len()
            );
            if let Some(subject) = subjects.first() {
                println!("subject bounds: {:?}", subject.bounds()?);
            }
            match interaction.image_for_subjects(&subjects) {
                Ok(image) => println!("subject image bytes: {}", image.png_data.len()),
                Err(error) => println!("subject image bytes: {error}"),
            }
        }
        Err(error) => println!("subjects: {error}"),
    }
    Ok(())
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

    let delegate = build_delegate()?;
    let interaction = LiveTextInteraction::with_delegate(&delegate)?;
    let tracking_view = LiveTextTrackingImageView::new()?;
    tracking_view.set_image_at_path(&asset_path)?;
    interaction.set_tracking_image_view(Some(&tracking_view))?;
    interaction.track_image_at_path(&asset_path)?;
    interaction.set_analysis(&analysis)?;
    interaction.set_preferred_interaction_types(LiveTextInteractionTypes::AUTOMATIC_TEXT_ONLY)?;
    interaction.set_selectable_items_highlighted(true)?;
    interaction.set_contents_rect_needs_update()?;

    println!("contents rect: {:?}", interaction.contents_rect()?);
    println!("overlay text: {:?}", interaction.text());
    println!("delegate events: {}", delegate.recorded_events()?.len());
    println!(
        "delegate content view set: {}",
        interaction.delegate()?.and_then(|value| value.content_view().ok()).flatten().is_some()
    );
    match interaction.tracking_image_view()? {
        Some(view) => println!("tracking image size: {:?}", view.image_size()?),
        None => println!("tracking image size: none"),
    }
    match LiveTextMenuTag::copy_image() {
        Ok(tag) => println!("copy image tag: {}", tag.raw_value()),
        Err(error) => println!("copy image tag: {error}"),
    }
    print_selection_state(&interaction)?;
    print_subject_state(&interaction)?;
    println!(
        "live text button visible: {}",
        interaction.live_text_button_visible()?
    );
    println!(
        "supplementary hidden: {}",
        interaction.is_supplementary_interface_hidden()?
    );

    let image_for_subjects_fn: fn(&LiveTextInteraction, &[LiveTextSubject]) -> Result<
        LiveTextImageData,
        VisionKitError,
    > = LiveTextInteraction::image_for_subjects;
    black_box(image_for_subjects_fn);
    Ok(())
}
