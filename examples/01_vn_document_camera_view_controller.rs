use visionkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = VNDocumentCameraViewController::support_info()?;
    assert!(!info.available_on_current_platform);
    println!("{} -> {}", info.area, info.availability);
    println!("reason: {}", info.reason.as_deref().unwrap_or("n/a"));
    Ok(())
}
