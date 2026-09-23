# visionkit-rs

Safe Rust bindings for Apple's `VisionKit.framework` on macOS.

> **Status:** v0.3.0 adds a Tier-1 async API module on top of full v0.2.x coverage of `ImageAnalysisOverlayView`, `ImageAnalyzer`, `ImageAnalysis`, and availability metadata for the iOS-only areas.

## Quick start

```rust,no_run
use visionkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !ImageAnalyzer::is_supported() {
        println!("ImageAnalyzer is not supported on this Mac");
        return Ok(());
    }

    let analyzer = ImageAnalyzer::new()?;
    let configuration = ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT)
        .with_locales(["en-US"]);
    let analysis = analyzer.analyze_image_at_path(
        "examples/assets/live_text.png",
        ImageOrientation::Up,
        &configuration,
    )?;

    let interaction = LiveTextInteraction::new()?;
    interaction.track_image_at_path("examples/assets/live_text.png")?;
    interaction.set_analysis(&analysis)?;

    println!("transcript: {}", analysis.transcript()?);
    println!("live text overlay text: {}", interaction.text()?);
    Ok(())
}
```

## Async API (`async` feature)

Enable with `visionkit = { version = "0.3", features = ["async"] }`.

```rust,no_run
use visionkit::async_api::{block_on, AsyncImageAnalyzer};
use visionkit::{ImageAnalysisTypes, ImageAnalyzerConfiguration, ImageOrientation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !AsyncImageAnalyzer::is_supported() {
        return Ok(());
    }
    let result = block_on(async {
        let cfg = ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT);
        AsyncImageAnalyzer::new()?
            .analyze_image_at_path("examples/assets/live_text.png", ImageOrientation::Up, &cfg)?
            .await
    });
    println!("transcript: {}", result?.transcript()?);
    Ok(())
}
```

**Note:** VisionKit completes image analysis through the main queue. On the main thread `block_on` pumps the Obj-C main run loop until the future is woken; on other threads it parks, and the futures only complete while the main thread runs its run loop (an AppKit app, or a CLI whose main thread calls `CFRunLoopRun`). The futures also work with any other executor (Tokio, async-std, …) under the same condition.

## Highlights

- `ImageAnalyzer::is_supported` and `supported_text_recognition_languages`
- `ImageAnalyzerConfiguration`, `ImageAnalysisTypes`, and `ImageOrientation`
- File-path based analysis through every public macOS analyzer overload: URL, `NSImage`, `CGImage`, `CIImage`, and `CVPixelBuffer`
- `ImageAnalysis::transcript` and `has_results`
- full `LiveTextInteraction` coverage for macOS `ImageAnalysisOverlayView`, including delegates, menu tags, selection metadata, tracking/content views, fonts, and subject analysis
- `VNDocumentCameraViewController`, `DataScannerViewController`, `RecognizedText`, `Barcode`, and `RecognizedItem` availability metadata on macOS

## Availability

- `ImageAnalyzer`, `ImageAnalysis`, and `LiveTextInteraction` are available on macOS 13+.
- `LiveTextInteraction::text`, `selected_text`, `selected_attributed_text`, `selected_ranges`, and `LiveTextMenuTag` require macOS 14+ because Apple introduced those overlay accessors after macOS 13.
- `VNDocumentCameraViewController`, `DataScannerViewController`, `RecognizedText`, `Barcode`, and `RecognizedItem` are iOS-only Apple APIs. On macOS, this crate exposes them as structured availability metadata instead of pretending they exist.

## Examples

```bash
cargo run --example 01_vn_document_camera_view_controller
cargo run --example 02_framework_smoke
cargo run --example 03_data_scanner_view_controller
cargo run --example 04_image_analyzer
cargo run --example 05_live_text_interaction
cargo run --example 06_image_analysis
cargo run --example 07_recognized_text
cargo run --example 08_barcode
cargo run --example 09_recognized_item
cargo run --features async --example 10_async_analyze
```

See [`COVERAGE.md`](COVERAGE.md) for the audited VisionKit API matrix.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
