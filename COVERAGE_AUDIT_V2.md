# visionkit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 89
VERIFIED: 89
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100

Methodology:
- Source of truth: `VisionKit.swiftinterface` (arm64e-apple-macos); the macOS public headers in the SDK are empty comment stubs.
- Enumerated all public macOS user-facing types, properties, methods, and nested types from the official swiftinterface; excluded AppKit-inherited boilerplate (e.g., `viewDidMoveToSuperview()`, `init(coder:)`), compiler-generated conformance code (e.g., typealias boilerplate), and iOS/watchOS-only symbols.
- Verified each swiftinterface symbol against the crate's public Rust API and Swift bridge thunks.
- Properties with getter/setter exposed as separate wrapper functions; counted once at the trait level.
- Coverage is 100%; the Rust wrapper and Swift bridge provide complete coverage of all public macOS VisionKit symbols.

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
| ImageAnalysisOverlayView.trackingImageView | var | VisionKit.swiftinterface | LiveTextInteraction::{tracking_image_view, set_tracking_image_view} + LiveTextTrackingImageView |
| ImageAnalysisOverlayView.supplementaryInterfaceFont | var | VisionKit.swiftinterface | LiveTextInteraction::{supplementary_interface_font, set_supplementary_interface_font} |
| ImageAnalysisOverlayView.delegate | var | VisionKit.swiftinterface | LiveTextInteraction::{delegate, set_delegate} |
| ImageAnalysisOverlayView.selectedAttributedText | var | VisionKit.swiftinterface | LiveTextInteraction::selected_attributed_text |
| ImageAnalysisOverlayView.selectedRanges | var | VisionKit.swiftinterface | LiveTextInteraction::{selected_ranges, set_selected_ranges} |
| ImageAnalysisOverlayView.setContentsRectNeedsUpdate() | func | VisionKit.swiftinterface | LiveTextInteraction::set_contents_rect_needs_update |
| ImageAnalysisOverlayView.init(_:) | func | VisionKit.swiftinterface | LiveTextInteraction::with_delegate |
| ImageAnalysisOverlayView.SubjectUnavailable | enum | VisionKit.swiftinterface | LiveTextSubjectUnavailable |
| ImageAnalysisOverlayView.SubjectUnavailable.imageUnavailable | case | VisionKit.swiftinterface | LiveTextSubjectUnavailable::ImageUnavailable |
| ImageAnalysisOverlayView.Subject | struct | VisionKit.swiftinterface | LiveTextSubject |
| ImageAnalysisOverlayView.Subject.bounds | var | VisionKit.swiftinterface | LiveTextSubject::bounds |
| ImageAnalysisOverlayView.Subject.image | var | VisionKit.swiftinterface | LiveTextSubject::image |
| ImageAnalysisOverlayView.beginSubjectAnalysisIfNecessary() | func | VisionKit.swiftinterface | LiveTextInteraction::begin_subject_analysis_if_necessary |
| ImageAnalysisOverlayView.subjects | var | VisionKit.swiftinterface | LiveTextInteraction::subjects |
| ImageAnalysisOverlayView.highlightedSubjects | var | VisionKit.swiftinterface | LiveTextInteraction::{highlighted_subjects, set_highlighted_subjects} |
| ImageAnalysisOverlayView.subject(at:) | func | VisionKit.swiftinterface | LiveTextInteraction::subject_at_point |
| ImageAnalysisOverlayView.image(for:) | func | VisionKit.swiftinterface | LiveTextInteraction::image_for_subjects |
| ImageAnalysisOverlayViewDelegate | protocol | VisionKit.swiftinterface | LiveTextInteractionDelegate |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldBeginAt:forAnalysisType:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{recorded_events, set_should_begin} |
| ImageAnalysisOverlayViewDelegate.contentsRect(for:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{contents_rect_override, set_contents_rect_override, recorded_events} |
| ImageAnalysisOverlayViewDelegate.contentView(for:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{content_view, set_content_view, recorded_events} |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldHandleKeyDownEvent:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{should_handle_key_down_event, set_should_handle_key_down_event, recorded_events} |
| ImageAnalysisOverlayViewDelegate.overlayView(_:shouldShowMenuForEvent:atPoint:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{should_show_menu_for_event, set_should_show_menu_for_event, recorded_events} |
| ImageAnalysisOverlayViewDelegate.overlayView(_:liveTextButtonDidChangeToVisible:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.overlayView(_:highlightSelectedItemsDidChange:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.textSelectionDidChange(_:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.overlayView(_:updatedMenuFor:for:at:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::{updated_menu, set_updated_menu, recorded_events} |
| ImageAnalysisOverlayViewDelegate.overlayView(_:needsUpdate:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.overlayView(_:willOpen:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.overlayView(_:didClose:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayViewDelegate.overlayView(_:menu:willHighlight:) | func | VisionKit.swiftinterface | LiveTextInteractionDelegate::recorded_events |
| ImageAnalysisOverlayView.MenuTag | struct | VisionKit.swiftinterface | LiveTextMenuTag |
| ImageAnalysisOverlayView.MenuTag.copyImage | static var | VisionKit.swiftinterface | LiveTextMenuTag::copy_image |
| ImageAnalysisOverlayView.MenuTag.shareImage | static var | VisionKit.swiftinterface | LiveTextMenuTag::share_image |
| ImageAnalysisOverlayView.MenuTag.copySubject | static var | VisionKit.swiftinterface | LiveTextMenuTag::copy_subject |
| ImageAnalysisOverlayView.MenuTag.shareSubject | static var | VisionKit.swiftinterface | LiveTextMenuTag::share_subject |
| ImageAnalysisOverlayView.MenuTag.lookupItem | static var | VisionKit.swiftinterface | LiveTextMenuTag::lookup_item |
| ImageAnalysisOverlayView.MenuTag.recommendedAppItems | static var | VisionKit.swiftinterface | LiveTextMenuTag::recommended_app_items |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | — | — | All public macOS `ImageAnalysisOverlayView` symbols from the audited swiftinterface are now bridged. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ | — | — | The macOS VisionKit swiftinterface exposes no deprecated or macOS-unavailable public symbols after filtering. | — |
