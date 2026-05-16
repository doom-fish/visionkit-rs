use visionkit::{prelude::*, VisionKitError};

#[test]
fn document_camera_reports_ios_only_unavailability() -> Result<(), Box<dyn std::error::Error>> {
    let info = VNDocumentCameraViewController::support_info()?;
    assert_eq!(info.area, "VNDocumentCameraViewController");
    assert!(!info.available_on_current_platform);
    assert!(info.availability.contains("iOS 13"));
    assert!(info
        .members
        .iter()
        .any(|member| member.contains("delegate")));
    assert!(matches!(
        VNDocumentCameraViewController::new(),
        Err(VisionKitError::UnavailableOnThisPlatform(_))
    ));
    Ok(())
}
