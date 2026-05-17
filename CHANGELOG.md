# Changelog

## 0.3.1

- fixed broken intra-doc links to `ImageAnalysisTypes::VISUAL_LOOK_UP` in `async_api` module
- improved `unsafe impl Send` and `unsafe impl Sync` safety documentation for `AsyncImageAnalyzer` and `AsyncOverlaySubjects`

## 0.3.0

- added `async_api` module (Tier 1) gated behind the `async` Cargo feature
- `AsyncImageAnalyzer::analyze_image_at_path` wraps `ImageAnalyzer.analyze(imageAt:orientation:configuration:) async throws` as a Rust `Future`
- `AsyncOverlaySubjects` wraps `ImageAnalysisOverlayView.subjects` and `.subject(at:)` as `SubjectsFuture` / `SubjectAtFuture`
- `AnalysisSubjectBounds` — JSON-serialisable bounds struct for `Subject` on macOS
- `async_api::block_on` — run-loop-aware executor that pumps the Obj-C main run loop between polls, required when calling VisionKit futures from the main thread
- Swift `@_cdecl` thunks use `Task { @MainActor in }` for the actual Apple API calls and fire C callbacks upon completion
- `vk_pump_main_run_loop(millis)` C hook for cross-language run-loop co-operation
- added `doom-fish-utils` optional dependency and `pollster` dev-dependency


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
