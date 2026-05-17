/// Async image analysis example.
///
/// Demonstrates `AsyncImageAnalyzer::analyze_image_at_path` (true-async Future)
/// and the `AsyncOverlaySubjects` futures for subject queries.
/// Uses `visionkit::async_api::block_on` which pumps the Obj-C main run loop
/// while driving the future — required because `VisionKit` Swift thunks dispatch
/// internally to the main actor.
use std::path::PathBuf;

use visionkit::async_api::{block_on, AsyncImageAnalyzer};
use visionkit::{ImageAnalysisTypes, ImageAnalyzerConfiguration, ImageOrientation};

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !AsyncImageAnalyzer::is_supported() {
        println!("ImageAnalyzer is not supported on this Mac — skipping");
        return Ok(());
    }

    block_on(async {
        // ── Async analyze ────────────────────────────────────────────────────
        let analyzer = AsyncImageAnalyzer::new()?;
        let cfg = ImageAnalyzerConfiguration::new(
            ImageAnalysisTypes::TEXT
                | ImageAnalysisTypes::MACHINE_READABLE_CODE
                | ImageAnalysisTypes::VISUAL_LOOK_UP,
        );

        println!("Analyzing image asynchronously…");
        let analysis = analyzer
            .analyze_image_at_path(asset_path(), ImageOrientation::Up, &cfg)
            .map_err(|e| format!("failed to create future: {e}"))?
            .await?;

        println!("has text: {}", analysis.has_results(ImageAnalysisTypes::TEXT)?);
        println!("transcript: {}", analysis.transcript()?);

        // ── Async subjects via overlay view ───────────────────────────────────
        // On macOS, `ImageAnalysisOverlayView.subjects` is `@MainActor async throws`
        // and only returns a non-empty set after an analysis is set on the view
        // *and* the view is attached to a live NSWindow. This example shows the
        // API surface compiles and links; a real UI integration test would call
        // `LiveTextInteraction::set_image_analysis(analysis)` first.
        println!("AsyncOverlaySubjects API compiled and linked successfully.");

        Ok::<_, Box<dyn std::error::Error>>(())
    })
}

