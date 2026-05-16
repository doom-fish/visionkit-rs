use visionkit::prelude::*;

#[test]
fn recognized_text_reports_ios_only_unavailability() -> Result<(), Box<dyn std::error::Error>> {
    let info = RecognizedText::support_info()?;
    assert_eq!(info.area, "RecognizedText");
    assert!(!info.available_on_current_platform);
    assert!(info.members.iter().any(|member| member == "transcript"));
    Ok(())
}
