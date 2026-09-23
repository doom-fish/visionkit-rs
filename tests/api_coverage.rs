use std::ffi::c_void;
use std::mem::size_of;

use visionkit::{analyzer, prelude::*};

#[test]
fn compatibility_reexports_still_exist() {
    assert_eq!(analyzer::ImageAnalysisTypes::TEXT, ImageAnalysisTypes::TEXT);
    assert_eq!(analyzer::ImageAnalysisTypes::TEXT.bits(), 1);
    assert_eq!(analyzer::ImageOrientation::Up.raw_value(), 1);
}

#[test]
fn prelude_exports_requested_area_types() {
    let handle = size_of::<*mut c_void>();
    assert_eq!(size_of::<ImageAnalysis>(), handle);
    assert_eq!(size_of::<ImageAnalyzer>(), handle);
    assert_eq!(size_of::<LiveTextInteraction>(), handle);
    assert_eq!(size_of::<LiveTextInteractionDelegate>(), handle);
    assert_eq!(size_of::<LiveTextSubject>(), handle);
}

#[test]
fn ios_only_areas_are_metadata_without_a_wrapped_object() {
    assert_eq!(size_of::<Barcode>(), 0);
    assert_eq!(size_of::<DataScannerViewController>(), 0);
    assert_eq!(size_of::<RecognizedItem>(), 0);
    assert_eq!(size_of::<RecognizedText>(), 0);
    assert_eq!(size_of::<VNDocumentCameraViewController>(), 0);
}
