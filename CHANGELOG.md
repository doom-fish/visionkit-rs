# Changelog

All notable changes to `visionkit-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- Fixed a use-after-free reachable from safe code: dropping a `LiveTextInteraction`
  while an `AsyncOverlaySubjects` query was pending let the main-actor Task borrow a
  freed overlay. The Swift thunks now take a strong reference to the overlay before
  they spawn the Task.

### Fixed

- Image analysis no longer runs inside a main-actor Task. Sync and async analysis
  run in a plain Task, and a timed-out Task is cancelled. VisionKit itself still
  finishes its work on the main queue, so an off-main synchronous call now fails
  after 10 seconds with a `TimedOut` error naming the main thread when nothing
  services the main queue, instead of timing out after 60 seconds.
- `async_api::block_on` no longer spins at 100% CPU off the main thread. It parks
  until woken, and pumps the main run loop only on the main thread.
- Async errors keep their type: the futures map the bridge status to the same
  `VisionKitError` variants as the sync API instead of `Unknown`.
- The analysis future no longer moves a non-`Send` `ImageAnalysis` across threads
  (doom-fish-utils 0.4.1 requires `Send` completion values); `AnalyzeImageFuture`,
  `SubjectsFuture` and `SubjectAtFuture` are now `Send`.
- `LiveTextInteraction::set_selected_ranges` returns `InvalidArgument` for a range
  whose end overflows, instead of aborting the process.

### Changed

- **Breaking:** `AsyncOverlaySubjects` is now `AsyncOverlaySubjects<'a>`: it borrows
  the `LiveTextInteraction` and is no longer `Send`.
- **Breaking:** the raw async callback type `ffi::image_analyzer::VkAsyncCb` takes an
  extra `i32` status argument.
- `rust-version` is 1.82 (was 1.76), and `doom-fish-utils` is required at
  `>=0.4.1, <0.5`.
- The iOS-only types document themselves as availability metadata, not wrappers.
  COVERAGE_AUDIT and V2 list four PARTIAL rows (path-only `NSImage` and `CIImage`
  overloads, the zero-frame overlay initializer, and the setter-only `analysis`), so
  coverage is 85/89 rather than 89/89, and they note the 26.5 re-check.

### Added

- `apple-cf` feature: `ImageAnalyzer::analyze_cg_image`, `ImageAnalyzer::analyze_pixel_buffer`,
  `AsyncImageAnalyzer::analyze_cg_image` and `AsyncImageAnalyzer::analyze_pixel_buffer`
  analyze in-memory `apple_cf` images, backed by new `vk_image_analyzer_analyze_cg_image*`
  and `vk_image_analyzer_analyze_pixel_buffer*` exports.
- README installation and threading sections.

## [0.3.6] - 2026-06-06

- Guarded the async `@MainActor` callbacks against panics crossing the FFI boundary.

## [0.3.5] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.4] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.3] - 2026-05-18

- added concise public-item doc comments across the non-FFI VisionKit wrappers and raised rustdoc coverage above the release target

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

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
