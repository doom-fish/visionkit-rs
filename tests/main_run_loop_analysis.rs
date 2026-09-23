use std::ffi::c_void;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use visionkit::prelude::*;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(mode: *const c_void, seconds: f64, return_after_source: u8) -> i32;
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("live_text.png")
}

fn configuration() -> ImageAnalyzerConfiguration {
    ImageAnalyzerConfiguration::new(ImageAnalysisTypes::TEXT)
}

fn pump_main_run_loop_until_finished<T>(handle: &JoinHandle<T>) {
    while !handle.is_finished() {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.01, 0) };
    }
}

fn assert_smoke_transcript(transcript: &str) {
    let transcript = transcript.to_lowercase();
    assert!(transcript.contains("visionkit") || transcript.contains("smoke"));
}

fn sync_analysis_off_main() {
    let worker = thread::spawn(|| {
        let started = Instant::now();
        let analysis = ImageAnalyzer::new()?.analyze_image_at_path(
            asset_path(),
            ImageOrientation::Up,
            &configuration(),
        )?;
        Ok::<_, VisionKitError>((analysis.transcript()?, started.elapsed()))
    });
    pump_main_run_loop_until_finished(&worker);
    let (transcript, elapsed) = worker
        .join()
        .expect("analysis thread panicked")
        .expect("off-main analysis failed");
    assert!(elapsed < Duration::from_secs(30));
    assert_smoke_transcript(&transcript);
}

#[cfg(feature = "async")]
fn async_analysis_off_main() {
    use visionkit::async_api::{block_on, AsyncImageAnalyzer};

    let worker = thread::spawn(|| {
        let analyzer = AsyncImageAnalyzer::new()?;
        let future =
            analyzer.analyze_image_at_path(asset_path(), ImageOrientation::Up, &configuration())?;
        block_on(future)?.transcript()
    });
    pump_main_run_loop_until_finished(&worker);
    let transcript = worker
        .join()
        .expect("async analysis thread panicked")
        .expect("off-main async analysis failed");
    assert_smoke_transcript(&transcript);
}

fn main() {
    if !ImageAnalyzer::is_supported() {
        println!("[skip] ImageAnalyzer is not supported on this Mac");
        return;
    }
    sync_analysis_off_main();
    #[cfg(feature = "async")]
    async_analysis_off_main();
    println!("off-main analysis completed while the main thread ran its run loop");
}
