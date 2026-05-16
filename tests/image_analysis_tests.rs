use std::path::PathBuf;

#[test]
fn transcript_and_has_results_work() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("examples")
            .join("06_image_analysis"),
    )
    .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    if stdout.contains("not supported on this mac") {
        return Ok(());
    }

    assert!(stdout.contains("has text results: true"));
    assert!(stdout.contains("visionkit") || stdout.contains("smoke"));
    Ok(())
}
