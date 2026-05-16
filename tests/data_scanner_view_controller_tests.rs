use visionkit::{prelude::*, VisionKitError};

#[test]
fn data_scanner_reports_ios_only_unavailability() -> Result<(), Box<dyn std::error::Error>> {
    let info = DataScannerViewController::support_info()?;
    assert_eq!(info.area, "DataScannerViewController");
    assert!(!info.available_on_current_platform);
    assert!(info.availability.contains("iOS 16"));
    assert!(info
        .members
        .iter()
        .any(|member| member.contains("capturePhoto")));
    assert!(matches!(
        DataScannerViewController::new(),
        Err(VisionKitError::UnavailableOnThisPlatform(_))
    ));
    Ok(())
}
