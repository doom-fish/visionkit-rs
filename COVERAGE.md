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

## ✅ Newly completed macOS surface

| Area | Apple API row | Status | Notes |
| --- | --- | --- | --- |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `ImageAnalysisOverlayViewDelegate` + `init(_ delegate:)` + `delegate` | ✅ implemented | `LiveTextInteractionDelegate`, `LiveTextInteraction::with_delegate`, and `LiveTextInteraction::{delegate,set_delegate}` bridge the full public delegate family with recorded callbacks and headless-safe menu/view models. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `MenuTag` + menu tag constants | ✅ implemented | `LiveTextMenuTag::{copy_image, share_image, copy_subject, share_subject, lookup_item, recommended_app_items}` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `trackingImageView` | ✅ implemented | `LiveTextTrackingImageView` plus `LiveTextInteraction::{tracking_image_view,set_tracking_image_view}` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `selectedAttributedText` / `selectedRanges` | ✅ implemented | `LiveTextInteraction::{selected_attributed_text, selected_ranges, set_selected_ranges}` return serializable attributed text runs and UTF-16 ranges. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `setContentsRectNeedsUpdate()` | ✅ implemented | `LiveTextInteraction::set_contents_rect_needs_update` |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `supplementaryInterfaceFont` | ✅ implemented | `LiveTextInteraction::{supplementary_interface_font, set_supplementary_interface_font}` use `LiveTextFont`. |
| LiveTextInteraction (`ImageAnalysisOverlayView`) | `SubjectUnavailable`, `Subject`, `beginSubjectAnalysisIfNecessary()`, `subjects`, `highlightedSubjects`, `subject(at:)`, `image(for:)` | ✅ implemented | `LiveTextSubjectUnavailable`, `LiveTextSubject`, `LiveTextInteraction::{begin_subject_analysis_if_necessary, subjects, highlighted_subjects, set_highlighted_subjects, subject_at_point, image_for_subjects}` and PNG image extraction. |

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
- `visionkit::LiveTextInteraction` / `LiveTextInteractionDelegate` / `LiveTextSubject` → macOS `VisionKit.ImageAnalysisOverlayView`
- `visionkit::VNDocumentCameraViewController` → availability metadata for the iOS-only document camera area
- `visionkit::DataScannerViewController` → availability metadata for the iOS-only data scanner area
- `visionkit::RecognizedText` / `Barcode` / `RecognizedItem` → availability metadata for the iOS-only recognized-item family
