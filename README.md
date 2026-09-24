# visionkit-rs

Safe Rust bindings for Apple's `VisionKit.framework` on macOS.

> **Status:** wraps the macOS `ImageAnalyzer`, `ImageAnalysis`, and `ImageAnalysisOverlayView` surface; four rows are partial (see [`COVERAGE.md`](COVERAGE.md)). The iOS-only areas are availability metadata only.

## Installation

```toml
[dependencies]
visionkit-rs = "0.4"
```

The library is imported as `visionkit`. Optional features:

- `async`: executor-agnostic futures in `visionkit::async_api`.
- `apple-cf`: analyze in-memory `apple_cf::cg::CGImage` and `apple_cf::cv::CVPixelBuffer` values.

```toml
visionkit-rs = { version = "0.4", features = ["async", "apple-cf"] }
```

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

Enable with `visionkit-rs = { version = "0.4", features = ["async"] }`.

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
- In-memory `CGImage` and `CVPixelBuffer` analysis (`ImageAnalyzer::analyze_cg_image`, `analyze_pixel_buffer`, and their async counterparts) with the `apple-cf` feature
- `ImageAnalysis::transcript` and `has_results`
- `LiveTextInteraction` coverage for macOS `ImageAnalysisOverlayView`, including delegates, menu tags, selection metadata, tracking/content views, fonts, and subject analysis. Delegate calls are answered from a configuration you set and recorded as events you poll; they do not call back into Rust.
- `VNDocumentCameraViewController`, `DataScannerViewController`, `RecognizedText`, `Barcode`, and `RecognizedItem` availability metadata on macOS. These iOS-only APIs are not wrapped: their constructors always fail on macOS.

## Threading

- VisionKit finishes `ImageAnalyzer` work on the main queue, although its Swift API is not bound to the main actor. A synchronous analysis on the main thread pumps the main run loop while it waits. On any other thread it needs the main thread to run its run loop (an AppKit app, or a CLI whose main thread calls `CFRunLoopRun`). If the main queue is not serviced within 10 seconds, the call returns `VisionKitError::MainRunLoopNotRunning`. Any analysis times out after 60 seconds with `VisionKitError::TimedOut`.
- `#[tokio::main]` and other executors that block the main thread starve VisionKit. Run the executor on another thread and keep the main thread in its run loop, or call the synchronous API from the main thread.
- `LiveTextInteraction` and its companion types wrap main-actor AppKit views and must be created on the main thread: their constructors return `VisionKitError::NotOnMainThread` elsewhere. The types are not `Send`, so they stay on the main thread, and every call runs inline there. Subject queries pump the main run loop while they wait and time out after 10 seconds.

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
