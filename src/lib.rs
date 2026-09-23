#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

/// Re-exports the core VisionKit analyzer types.
pub mod analyzer;
/// Availability metadata for the iOS-only barcode area.
pub mod barcode;
/// Availability metadata for the iOS-only data scanner.
pub mod data_scanner_view_controller;
/// Exposes error types returned by the VisionKit wrappers.
pub mod error;
/// Exposes the raw VisionKit bridge layer.
pub mod ffi;
/// Exposes the VisionKit image analysis wrapper.
pub mod image_analysis;
/// Exposes the VisionKit image analyzer wrapper and configuration types.
pub mod image_analyzer;
/// Exposes VisionKit live text and overlay wrappers.
pub mod live_text_interaction;
mod private;
/// Availability metadata for the iOS-only recognized-item area.
pub mod recognized_item;
/// Availability metadata for the iOS-only recognized-text area.
pub mod recognized_text;
/// Exposes VisionKit availability metadata types.
pub mod support;
/// Availability metadata for the iOS-only document camera.
pub mod vn_document_camera_view_controller;

#[cfg(feature = "async")]
/// Exposes executor-agnostic async VisionKit wrappers.
pub mod async_api;

pub use barcode::Barcode;
pub use data_scanner_view_controller::DataScannerViewController;
pub use error::{LiveTextSubjectUnavailable, VisionKitError};
pub use image_analysis::ImageAnalysis;
pub use image_analyzer::{
    ImageAnalysisTypes, ImageAnalyzer, ImageAnalyzerConfiguration, ImageOrientation,
};
pub use live_text_interaction::{
    EdgeInsets, LiveTextAttributedText, LiveTextAttributedTextAttribute, LiveTextAttributedTextRun,
    LiveTextContentView, LiveTextDelegateEvent, LiveTextEventInfo, LiveTextFont, LiveTextImageData,
    LiveTextInteraction, LiveTextInteractionDelegate, LiveTextInteractionTypes, LiveTextMenu,
    LiveTextMenuItem, LiveTextMenuTag, LiveTextSubject, LiveTextTextRange,
    LiveTextTrackingImageView, Point, Rect, Size,
};
pub use recognized_item::RecognizedItem;
pub use recognized_text::RecognizedText;
pub use support::AreaSupportInfo;
pub use vn_document_camera_view_controller::VNDocumentCameraViewController;

/// Re-exports the core VisionKit wrappers for convenient imports.
pub mod prelude {
    pub use crate::barcode::Barcode;
    pub use crate::data_scanner_view_controller::DataScannerViewController;
    pub use crate::error::{LiveTextSubjectUnavailable, VisionKitError};
    pub use crate::image_analysis::ImageAnalysis;
    pub use crate::image_analyzer::{
        ImageAnalysisTypes, ImageAnalyzer, ImageAnalyzerConfiguration, ImageOrientation,
    };
    pub use crate::live_text_interaction::{
        EdgeInsets, LiveTextAttributedText, LiveTextAttributedTextAttribute,
        LiveTextAttributedTextRun, LiveTextContentView, LiveTextDelegateEvent, LiveTextEventInfo,
        LiveTextFont, LiveTextImageData, LiveTextInteraction, LiveTextInteractionDelegate,
        LiveTextInteractionTypes, LiveTextMenu, LiveTextMenuItem, LiveTextMenuTag, LiveTextSubject,
        LiveTextTextRange, LiveTextTrackingImageView, Point, Rect, Size,
    };
    pub use crate::recognized_item::RecognizedItem;
    pub use crate::recognized_text::RecognizedText;
    pub use crate::support::AreaSupportInfo;
    pub use crate::vn_document_camera_view_controller::VNDocumentCameraViewController;
}
