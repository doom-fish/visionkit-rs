use visionkit::{analyzer, prelude::*};

#[test]
fn compatibility_reexports_still_exist() {
    let _ = analyzer::ImageAnalysisTypes::TEXT;
    let _ = analyzer::ImageOrientation::Up;
}

#[test]
fn prelude_exports_requested_area_types() {
    let _ = std::mem::size_of::<Barcode>();
    let _ = std::mem::size_of::<DataScannerViewController>();
    let _ = std::mem::size_of::<ImageAnalysis>();
    let _ = std::mem::size_of::<ImageAnalyzer>();
    let _ = std::mem::size_of::<LiveTextInteraction>();
    let _ = std::mem::size_of::<RecognizedItem>();
    let _ = std::mem::size_of::<RecognizedText>();
    let _ = std::mem::size_of::<VNDocumentCameraViewController>();
}
