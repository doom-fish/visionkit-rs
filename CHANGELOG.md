# Changelog

## 0.2.1

- completed the remaining macOS `ImageAnalysisOverlayView` audit gaps with delegate, menu-tag, selection-range, font, tracking-image-view, and subject-analysis wrappers
- added headless-safe Rust models for overlay delegates, menus, attributed selections, tracking/content views, subjects, and extracted PNG image data
- extended the live-text example/tests and closed the audit at 100% macOS surface coverage

## 0.2.0

- added per-area Rust modules and Swift bridge files for document camera, data scanner, image analyzer, live text interaction, image analysis, recognized text, barcode, and recognized item coverage
- extended `ImageAnalyzer` with path-driven wrappers for the `NSImage`, `CGImage`, `CIImage`, and `CVPixelBuffer` analyzer overloads
- added a headless-friendly `LiveTextInteraction` wrapper around macOS `ImageAnalysisOverlayView`
- added numbered examples, per-area tests, and an audited `COVERAGE.md`
- surfaced structured availability metadata for the iOS-only VisionKit areas on macOS

## 0.1.0

- initial release
- added `ImageAnalyzer` and `ImageAnalysis` wrappers for macOS VisionKit image analysis
- added text, machine-readable-code, and visual-lookup analysis type configuration
- added a bundled OCR smoke image and end-to-end framework smoke example
