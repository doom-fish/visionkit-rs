# VisionKit coverage audit

Audited against:

- macOS 26.2 `VisionKit.swiftinterface`
- iOS 26.2 `VisionKit.swiftinterface`
- iOS 26.2 `VNDocumentCameraViewController.h` / `VNDocumentCameraScan.h`

## ✅ Implemented macOS surface

| Area | Apple API row | Status | Notes |
| --- | --- | --- | --- |
| ImageAnalyzer | `init()` | ✅ implemented | `ImageAnalyzer::new` |
| ImageAnalyzer | `class var isSupported` | ✅ implemented | `ImageAnalyzer::is_supported` |
| ImageAnalyzer | `class var supportedTextRecognitionLanguages` | ✅ implemented | `ImageAnalyzer::supported_text_recognition_languages` |
| ImageAnalyzer | `analyze(imageAt:orientation:configuration:)` | ✅ implemented | `ImageAnalyzer::analyze_image_at_path` |
| ImageAnalyzer | `analyze(_ image: NSImage, orientation:, configuration:)` | ✅ implemented | `ImageAnalyzer::analyze_ns_image_at_path` loads the `NSImage` in Swift |
| ImageAnalyzer | `analyze(_ cgImage: CGImage, orientation:, configuration:)` | ✅ implemented | `ImageAnalyzer::analyze_cg_image_at_path` loads the `CGImage` in Swift |
| ImageAnalyzer | `analyze(_ ciImage: CIImage, orientation:, configuration:)` | ✅ implemented | `ImageAnalyzer::analyze_ci_image_at_path` loads the `CIImage` in Swift |
| ImageAnalyzer | `analyze(_ pixelBuffer: CVPixelBuffer, orientation:, configuration:)` | ✅ implemented | `ImageAnalyzer::analyze_pixel_buffer_at_path` converts the image into a pixel buffer in Swift |
| ImageAnalysis | `transcript` | ✅ implemented | `ImageAnalysis::transcript` |
| ImageAnalysis | `hasResults(for:)` | ✅ implemented | `ImageAnalysis::has_results` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `init(frame:)` | ✅ implemented | `LiveTextInteraction::new` constructs the overlay view |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `analysis` | ✅ implemented | `LiveTextInteraction::set_analysis` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `preferredInteractionTypes` | ✅ implemented | getter + setter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `activeInteractionTypes` | ✅ implemented | getter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `selectableItemsHighlighted` | ✅ implemented | getter + setter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `hasActiveTextSelection` | ✅ implemented | getter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `resetSelection()` | ✅ implemented | `LiveTextInteraction::reset_selection` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `text` | ✅ implemented | `LiveTextInteraction::text` (macOS 14+) |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `selectedText` | ✅ implemented | `LiveTextInteraction::selected_text` (macOS 14+) |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `contentsRect` | ✅ implemented | `LiveTextInteraction::contents_rect` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `hasInteractiveItem(at:)` | ✅ implemented | point query |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `hasText(at:)` | ✅ implemented | point query |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `hasDataDetector(at:)` | ✅ implemented | point query |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `hasSupplementaryInterface(at:)` | ✅ implemented | point query |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `analysisHasText(at:)` | ✅ implemented | point query |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `liveTextButtonVisible` | ✅ implemented | getter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `isSupplementaryInterfaceHidden` | ✅ implemented | getter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `setSupplementaryInterfaceHidden(_:animated:)` | ✅ implemented | setter |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `supplementaryInterfaceContentInsets` | ✅ implemented | getter + setter |

## 🟡 Partial macOS surface

| Area | Apple API row | Status | Notes |
| --- | --- | --- | --- |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `delegate` / `init(_ delegate:)` | 🟡 partial | Delegate callbacks are not surfaced yet; the wrapper focuses on headless-safe overlay state and inspection. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `setContentsRectNeedsUpdate()` | 🟡 partial | Not yet surfaced in Rust. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `trackingImageView` | 🟡 partial | Managed internally through `track_image_at_path`, not exposed as a raw AppKit view. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `selectedAttributedText` / `selectedRanges` | 🟡 partial | String-only access is exposed; attributed/range metadata is not yet bridged. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `supplementaryInterfaceFont` | 🟡 partial | Font objects are not bridged. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `beginSubjectAnalysisIfNecessary()` | 🟡 partial | Subject analysis trigger not surfaced yet. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `SubjectUnavailable`, `Subject`, `subjects`, `highlightedSubjects`, `subject(at:)`, `image(for:)` | 🟡 partial | Subject/image extraction APIs are not yet surfaced. |

## ⏭️ Skipped iOS-only surface

| Area | Apple API row | Status | Reason |
| --- | --- | --- | --- |
| VNDocumentCameraViewController | controller + delegate callbacks | ⏭️ skipped | Apple marks the document camera UI as `API_UNAVAILABLE(macos, tvos, watchos)`. |
| VNDocumentCameraViewController | `VNDocumentCameraScan` (`pageCount`, `imageOfPageAtIndex`, `title`) | ⏭️ skipped | Result type is part of the iOS-only document camera area. |
| DataScannerViewController | controller, delegate, camera capture, zoom, ROI, recognized-item stream | ⏭️ skipped | Apple ships Data Scanner as iOS-only VisionKit surface with no macOS implementation. |
| RecognizedItem | enum cases + bounds + identity | ⏭️ skipped | Nested under the iOS-only Data Scanner API. |
| RecognizedText | `RecognizedItem.Text` members | ⏭️ skipped | Nested under the iOS-only Data Scanner API. |
| Barcode | `RecognizedItem.Barcode` members | ⏭️ skipped | Nested under the iOS-only Data Scanner API. |

## Rust area mapping

- `visionkit::ImageAnalyzer` → `VisionKit.ImageAnalyzer`
- `visionkit::ImageAnalysis` → `VisionKit.ImageAnalysis`
- `visionkit::LiveTextInteraction` → macOS `VisionKit.ImageAnalysisOverlayView`
- `visionkit::VNDocumentCameraViewController` → availability metadata for the iOS-only document camera area
- `visionkit::DataScannerViewController` → availability metadata for the iOS-only data scanner area
- `visionkit::RecognizedText` / `Barcode` / `RecognizedItem` → availability metadata for the iOS-only recognized-item family
