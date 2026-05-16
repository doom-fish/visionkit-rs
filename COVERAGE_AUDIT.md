# visionkit-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 89
VERIFIED: 51
GAPS: 38
EXEMPT: 0
COVERAGE_PCT: 57.30%

Methodology:
- Source of truth: `VisionKit.framework/.../VisionKit.swiftinterface`; the macOS public headers in the SDK are empty comment stubs.
- Counted unique user-facing public types, vars, funcs, and enum cases from the macOS arm64e swiftinterface.
- Excluded duplicate default implementations in `extension ImageAnalysisOverlayViewDelegate`, compiler-generated conformance/typealias boilerplate, and inherited AppKit boilerplate such as `init(coder:)` / `viewDidMoveToSuperview()`.
- Properties count once; when the Rust API only exposes part of a property surface, the `Wrapped by` or `Notes` column calls that out explicitly.
- Crate-only availability helpers for iOS-only `DataScannerViewController`, `VNDocumentCameraViewController`, `RecognizedItem`, `RecognizedText`, and `Barcode` are outside the macOS swiftinterface and were excluded from the denominator.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| ImageAnalysis | class | VisionKit.swiftinterface | ImageAnalysis |
| ImageAnalysis.transcript | var | VisionKit.swiftinterface | ImageAnalysis::transcript |
| ImageAnalysis.hasResults(for:) | func | VisionKit.swiftinterface | ImageAnalysis::has_results |
| ImageAnalyzer | class | VisionKit.swiftinterface | ImageAnalyzer |
| ImageAnalyzer.Configuration | struct | VisionKit.swiftinterface | ImageAnalyzerConfiguration |
| ImageAnalyzer.Configuration.analysisTypes | var | VisionKit.swiftinterface | ImageAnalyzerConfiguration::analysis_types |
| ImageAnalyzer.Configuration.locales | var | VisionKit.swiftinterface | ImageAnalyzerConfiguration::{locales, with_locales} |
| ImageAnalyzer.Configuration.init(_:) | func | VisionKit.swiftinterface | ImageAnalyzerConfiguration::new |
| ImageAnalyzer.AnalysisTypes | struct | VisionKit.swiftinterface | ImageAnalysisTypes |
| ImageAnalyzer.AnalysisTypes.rawValue | var | VisionKit.swiftinterface | ImageAnalysisTypes::{bits, new} |
| ImageAnalyzer.AnalysisTypes.init(rawValue:) | func | VisionKit.swiftinterface | ImageAnalysisTypes::new |
| ImageAnalyzer.AnalysisTypes.text | static var | VisionKit.swiftinterface | ImageAnalysisTypes::TEXT |
| ImageAnalyzer.AnalysisTypes.machineReadableCode | static var | VisionKit.swiftinterface | ImageAnalysisTypes::MACHINE_READABLE_CODE |
| ImageAnalyzer.AnalysisTypes.visualLookUp | static var | VisionKit.swiftinterface | ImageAnalysisTypes::VISUAL_LOOK_UP |
| ImageAnalyzer.init() | func | VisionKit.swiftinterface | ImageAnalyzer::new |
| ImageAnalyzer.isSupported | class var | VisionKit.swiftinterface | ImageAnalyzer::is_supported |
| ImageAnalyzer.supportedTextRecognitionLanguages | class var | VisionKit.swiftinterface | ImageAnalyzer::supported_text_recognition_languages |
| ImageAnalyzer.analyze(_ image:orientation:configuration:) | func | VisionKit.swiftinterface | ImageAnalyzer::analyze_ns_image_at_path |
| ImageAnalyzer.analyze(_ cgImage:orientation:configuration:) | func | VisionKit.swiftinterface | ImageAnalyzer::analyze_cg_image_at_path |
| ImageAnalyzer.analyze(_ ciImage:orientation:configuration:) | func | VisionKit.swiftinterface | ImageAnalyzer::analyze_ci_image_at_path |
| ImageAnalyzer.analyze(_ pixelBuffer:orientation:configuration:) | func | VisionKit.swiftinterface | ImageAnalyzer::analyze_pixel_buffer_at_path |
| ImageAnalyzer.analyze(imageAt:orientation:configuration:) | func | VisionKit.swiftinterface | ImageAnalyzer::analyze_image_at_path |
| ImageAnalysisOverlayView | class | VisionKit.swiftinterface | LiveTextInteraction |
| ImageAnalysisOverlayView.InteractionTypes | struct | VisionKit.swiftinterface | LiveTextInteractionTypes |
| ImageAnalysisOverlayView.InteractionTypes.rawValue | var | VisionKit.swiftinterface | LiveTextInteractionTypes::{bits, new} |
| ImageAnalysisOverlayView.InteractionTypes.init(rawValue:) | func | VisionKit.swiftinterface | LiveTextInteractionTypes::new |
| ImageAnalysisOverlayView.InteractionTypes.automatic | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::AUTOMATIC |
| ImageAnalysisOverlayView.InteractionTypes.automaticTextOnly | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::AUTOMATIC_TEXT_ONLY |
| ImageAnalysisOverlayView.InteractionTypes.textSelection | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::TEXT_SELECTION |
| ImageAnalysisOverlayView.InteractionTypes.dataDetectors | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::DATA_DETECTORS |
| ImageAnalysisOverlayView.InteractionTypes.imageSubject | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::IMAGE_SUBJECT |
| ImageAnalysisOverlayView.InteractionTypes.visualLookUp | static var | VisionKit.swiftinterface | LiveTextInteractionTypes::VISUAL_LOOK_UP |
| ImageAnalysisOverlayView.init(frame:) | func | VisionKit.swiftinterface | LiveTextInteraction::new (constructs frame .zero) |
| ImageAnalysisOverlayView.analysis | var | VisionKit.swiftinterface | LiveTextInteraction::set_analysis (setter only) |
| ImageAnalysisOverlayView.preferredInteractionTypes | var | VisionKit.swiftinterface | LiveTextInteraction::{preferred_interaction_types, set_preferred_interaction_types} |
| ImageAnalysisOverlayView.activeInteractionTypes | var | VisionKit.swiftinterface | LiveTextInteraction::active_interaction_types |
| ImageAnalysisOverlayView.selectableItemsHighlighted | var | VisionKit.swiftinterface | LiveTextInteraction::{selectable_items_highlighted, set_selectable_items_highlighted} |
| ImageAnalysisOverlayView.hasActiveTextSelection | var | VisionKit.swiftinterface | LiveTextInteraction::has_active_text_selection |
| ImageAnalysisOverlayView.resetSelection() | func | VisionKit.swiftinterface | LiveTextInteraction::reset_selection |
| ImageAnalysisOverlayView.text | var | VisionKit.swiftinterface | LiveTextInteraction::text |
| ImageAnalysisOverlayView.selectedText | var | VisionKit.swiftinterface | LiveTextInteraction::selected_text |
| ImageAnalysisOverlayView.contentsRect | var | VisionKit.swiftinterface | LiveTextInteraction::contents_rect |
| ImageAnalysisOverlayView.hasInteractiveItem(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::has_interactive_item_at_point |
| ImageAnalysisOverlayView.hasText(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::has_text_at_point |
| ImageAnalysisOverlayView.hasDataDetector(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::has_data_detector_at_point |
| ImageAnalysisOverlayView.hasSupplementaryInterface(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::has_supplementary_interface_at_point |
| ImageAnalysisOverlayView.analysisHasText(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::analysis_has_text_at_point |
| ImageAnalysisOverlayView.liveTextButtonVisible | var | VisionKit.swiftinterface | LiveTextInteraction::live_text_button_visible |
| ImageAnalysisOverlayView.isSupplementaryInterfaceHidden | var | VisionKit.swiftinterface | LiveTextInteraction::{is_supplementary_interface_hidden, set_supplementary_interface_hidden} |
| ImageAnalysisOverlayView.setSupplementaryInterfaceHidden(_:animated:) | func | VisionKit.swiftinterface | LiveTextInteraction::set_supplementary_interface_hidden |
| ImageAnalysisOverlayView.supplementaryInterfaceContentInsets | var | VisionKit.swiftinterface | LiveTextInteraction::{supplementary_interface_content_insets, set_supplementary_interface_content_insets} |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| ImageAnalysisOverlayViewDelegate | protocol | VisionKit.swiftinterface | No public delegate protocol bridge. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldBeginAt:forAnalysisType:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.contentsRect(for:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.contentView(for:) | func | VisionKit.swiftinterface | No AppKit content view delegate bridge. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldHandleKeyDownEvent:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldShowMenuForEvent:atPoint:) | func | VisionKit.swiftinterface | No delegate callback or menu customization bridge. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:liveTextButtonDidChangeToVisible:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:highlightSelectedItemsDidChange:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.textSelectionDidChange(_:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:updatedMenuFor:for:at:) | func | VisionKit.swiftinterface | No NSMenu customization bridge. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:needsUpdate:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:willOpen:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:didClose:) | func | VisionKit.swiftinterface | No delegate callback surface. |
| ImageAnalysisOverlayViewDelegate.overlayView(_:menu:willHighlight:) | func | VisionKit.swiftinterface | No NSMenuItem delegate bridge. |
| ImageAnalysisOverlayView.MenuTag | struct | VisionKit.swiftinterface | Menu-tag helper type is not exported. |
| ImageAnalysisOverlayView.MenuTag.copyImage | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.MenuTag.shareImage | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.MenuTag.copySubject | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.MenuTag.shareSubject | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.MenuTag.lookupItem | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.MenuTag.recommendedAppItems | static var | VisionKit.swiftinterface | Menu tag constants are not exported. |
| ImageAnalysisOverlayView.init(_:) | func | VisionKit.swiftinterface | No delegate-based convenience initializer. |
| ImageAnalysisOverlayView.delegate | var | VisionKit.swiftinterface | No delegate getter/setter bridge. |
| ImageAnalysisOverlayView.trackingImageView | var | VisionKit.swiftinterface | Only managed internally via track_image_at_path; raw NSImageView is not exposed. |
| ImageAnalysisOverlayView.selectedAttributedText | var | VisionKit.swiftinterface | Only plain selected_text is exposed. |
| ImageAnalysisOverlayView.selectedRanges | var | VisionKit.swiftinterface | Selection range metadata is not exposed. |
| ImageAnalysisOverlayView.setContentsRectNeedsUpdate() | func | VisionKit.swiftinterface | Not bridged. |
| ImageAnalysisOverlayView.supplementaryInterfaceFont | var | VisionKit.swiftinterface | NSFont bridge is missing. |
| ImageAnalysisOverlayView.SubjectUnavailable | enum | VisionKit.swiftinterface | Subject-analysis error surface is not exported. |
| ImageAnalysisOverlayView.SubjectUnavailable.imageUnavailable | case | VisionKit.swiftinterface | Subject-analysis error surface is not exported. |
| ImageAnalysisOverlayView.Subject | struct | VisionKit.swiftinterface | Subject handles are not exported. |
| ImageAnalysisOverlayView.Subject.bounds | var | VisionKit.swiftinterface | Subject handles are not exported. |
| ImageAnalysisOverlayView.Subject.image | var | VisionKit.swiftinterface | Subject handles are not exported. |
| ImageAnalysisOverlayView.beginSubjectAnalysisIfNecessary() | func | VisionKit.swiftinterface | Subject analysis control is not bridged. |
| ImageAnalysisOverlayView.subjects | var | VisionKit.swiftinterface | Subject collection is not exposed. |
| ImageAnalysisOverlayView.highlightedSubjects | var | VisionKit.swiftinterface | Subject highlighting is not exposed. |
| ImageAnalysisOverlayView.subject(at:) | func | VisionKit.swiftinterface | Point lookup for subjects is not exposed. |
| ImageAnalysisOverlayView.image(for:) | func | VisionKit.swiftinterface | Subject image extraction is not exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ | — | — | The macOS VisionKit swiftinterface exposes no deprecated or macOS-unavailable public symbols after filtering. | — |
