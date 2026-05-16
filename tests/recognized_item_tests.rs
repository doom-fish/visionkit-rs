use visionkit::prelude::*;

#[test]
fn recognized_item_reports_ios_only_unavailability() -> Result<(), Box<dyn std::error::Error>> {
    let info = RecognizedItem::support_info()?;
    assert_eq!(info.area, "RecognizedItem");
    assert!(!info.available_on_current_platform);
    assert!(info
        .members
        .iter()
        .any(|member| member.contains("case text")));
    assert!(info
        .members
        .iter()
        .any(|member| member.contains("case barcode")));
    Ok(())
}
