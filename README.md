# visionkit-rs

Safe Rust bindings for Apple's `VisionKit.framework` on macOS.

> **Status:** v0.1.0 covers `ImageAnalyzer`, `ImageAnalysis`, text / machine-readable-code / visual-lookup analysis types, OCR language discovery, and file-path based image analysis on macOS.

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

    println!("{}", analysis.transcript()?);
    Ok(())
}
```

## Highlights

- `ImageAnalyzer::is_supported` and `supported_text_recognition_languages`
- `ImageAnalyzerConfiguration`, `ImageAnalysisTypes`, and `ImageOrientation`
- `ImageAnalyzer::analyze_image_at_path`
- `ImageAnalysis::transcript` and `has_results`

## Availability

- `ImageAnalyzer` / `ImageAnalysis` are available on macOS 13+.
- This crate guards the framework at runtime and returns a structured `VisionKitError` when the OS is too old or `ImageAnalyzer` is unsupported on the current Mac.
- This crate intentionally focuses on the macOS image-analysis APIs and does not expose iOS-only document-camera or Data Scanner surfaces.

## Smoke example

Run the framework smoke example with:

```bash
cargo run --all-features --example 02_framework_smoke
```

It prints analyzer support, OCR languages, analyzes the bundled `examples/assets/live_text.png`, and shows the extracted transcript.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
