use visionkit::prelude::*;

#[test]
fn barcode_reports_ios_only_unavailability() -> Result<(), Box<dyn std::error::Error>> {
    let info = Barcode::support_info()?;
    assert_eq!(info.area, "Barcode");
    assert!(!info.available_on_current_platform);
    assert!(info
        .members
        .iter()
        .any(|member| member == "payloadStringValue"));
    Ok(())
}
