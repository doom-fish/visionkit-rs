#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod analyzer;
pub mod barcode;
pub mod data_scanner_view_controller;
pub mod error;
pub mod ffi;
pub mod image_analysis;
pub mod image_analyzer;
pub mod live_text_interaction;
mod private;
pub mod recognized_item;
pub mod recognized_text;
pub mod support;
pub mod vn_document_camera_view_controller;

pub use barcode::Barcode;
pub use data_scanner_view_controller::DataScannerViewController;
pub use error::{LiveTextSubjectUnavailable, VisionKitError};
pub use image_analysis::ImageAnalysis;
pub use image_analyzer::{
    ImageAnalysisTypes, ImageAnalyzer, ImageAnalyzerConfiguration, ImageOrientation,
};
pub use live_text_interaction::{
    EdgeInsets, LiveTextAttributedText, LiveTextAttributedTextAttribute, LiveTextAttributedTextRun,
    LiveTextContentView, LiveTextDelegateEvent, LiveTextEventInfo, LiveTextFont,
    LiveTextImageData, LiveTextInteraction, LiveTextInteractionDelegate,
    LiveTextInteractionTypes, LiveTextMenu, LiveTextMenuItem, LiveTextMenuTag,
    LiveTextSubject, LiveTextTextRange, LiveTextTrackingImageView, Point, Rect, Size,
};
pub use recognized_item::RecognizedItem;
pub use recognized_text::RecognizedText;
pub use support::AreaSupportInfo;
pub use vn_document_camera_view_controller::VNDocumentCameraViewController;

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
        LiveTextAttributedTextRun, LiveTextContentView, LiveTextDelegateEvent,
        LiveTextEventInfo, LiveTextFont, LiveTextImageData, LiveTextInteraction,
        LiveTextInteractionDelegate, LiveTextInteractionTypes, LiveTextMenu,
        LiveTextMenuItem, LiveTextMenuTag, LiveTextSubject, LiveTextTextRange,
        LiveTextTrackingImageView, Point, Rect, Size,
    };
    pub use crate::recognized_item::RecognizedItem;
    pub use crate::recognized_text::RecognizedText;
    pub use crate::support::AreaSupportInfo;
    pub use crate::vn_document_camera_view_controller::VNDocumentCameraViewController;
}
