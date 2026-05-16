use std::path::PathBuf;

#[test]
fn live_text_interaction_round_trips_basic_state() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("examples")
            .join("05_live_text_interaction"),
    )
    .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    if stdout.contains("not supported on this mac") {
        return Ok(());
    }

    assert!(stdout.contains("contents rect:"));
    assert!(stdout.contains("live text button visible:"));
    Ok(())
}
