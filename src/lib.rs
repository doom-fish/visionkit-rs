#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod analyzer;
pub mod error;
pub mod ffi;
mod private;

pub use analyzer::{
    ImageAnalysis, ImageAnalysisTypes, ImageAnalyzer, ImageAnalyzerConfiguration, ImageOrientation,
};
pub use error::VisionKitError;

pub mod prelude {
    pub use crate::analyzer::{
        ImageAnalysis, ImageAnalysisTypes, ImageAnalyzer, ImageAnalyzerConfiguration,
        ImageOrientation,
    };
    pub use crate::error::VisionKitError;
}
