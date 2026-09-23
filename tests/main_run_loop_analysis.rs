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

#[cfg(feature = "apple-cf")]
fn white_image() -> apple_cf::cg::CGImage {
    let context = apple_cf::cg::CGContext::new_rgba8(320, 240).expect("bitmap context");
    context.set_rgb_fill_color(1.0, 1.0, 1.0, 1.0);
    context.fill_rect(0.0, 0.0, 320.0, 240.0);
    context.snapshot_to_image().expect("bitmap snapshot")
}

#[cfg(feature = "apple-cf")]
fn in_memory_analysis_on_main() {
    use std::ffi::CString;
    use std::ptr;

    use apple_cf::cv::CVPixelBuffer;
    use visionkit::ffi;

    let analyzer = ImageAnalyzer::new().expect("analyzer");
    let image = white_image();
    let analysis = analyzer
        .analyze_cg_image(&image, ImageOrientation::Up, &configuration())
        .expect("in-memory CGImage analysis");
    assert!(!analysis
        .has_results(ImageAnalysisTypes::TEXT)
        .expect("has_results"));
    assert!(analysis.transcript().expect("transcript").is_empty());

    let pixel_buffer = CVPixelBuffer::create(320, 240, 0x4247_5241).expect("pixel buffer");
    let analysis = analyzer
        .analyze_pixel_buffer(&pixel_buffer, ImageOrientation::Up, &configuration())
        .expect("in-memory CVPixelBuffer analysis");
    assert!(analysis.transcript().is_ok());

    let configuration_json = CString::new(r#"{"analysisTypes":1,"locales":[]}"#).expect("json");
    let token = unsafe { ffi::image_analyzer::vk_image_analyzer_new() };
    assert!(!token.is_null());
    let mut analysis_token = ptr::null_mut();
    let mut error_message = ptr::null_mut();
    let status = unsafe {
        ffi::image_analyzer::vk_image_analyzer_analyze_cg_image(
            token,
            pixel_buffer.as_ptr(),
            ImageOrientation::Up.raw_value(),
            configuration_json.as_ptr(),
            &mut analysis_token,
            &mut error_message,
        )
    };
    assert_eq!(status, ffi::status::INVALID_ARGUMENT);
    assert!(analysis_token.is_null());
    assert!(!error_message.is_null());
    unsafe {
        ffi::vk_string_free(error_message);
        ffi::image_analyzer::vk_image_analyzer_release(token);
    }
}

#[cfg(all(feature = "apple-cf", feature = "async"))]
fn async_in_memory_analysis_on_main() {
    use visionkit::async_api::{block_on, AsyncImageAnalyzer};

    let analyzer = AsyncImageAnalyzer::new().expect("async analyzer");
    let image = white_image();
    let future = analyzer
        .analyze_cg_image(&image, ImageOrientation::Up, &configuration())
        .expect("CGImage future");
    drop(image);
    let analysis = block_on(future).expect("async in-memory CGImage analysis");
    assert!(!analysis
        .has_results(ImageAnalysisTypes::TEXT)
        .expect("has_results"));

    let pixel_buffer =
        apple_cf::cv::CVPixelBuffer::create(320, 240, 0x4247_5241).expect("pixel buffer");
    let future = analyzer
        .analyze_pixel_buffer(&pixel_buffer, ImageOrientation::Up, &configuration())
        .expect("CVPixelBuffer future");
    drop(pixel_buffer);
    assert!(block_on(future).is_ok());
}

fn main() {
    if !ImageAnalyzer::is_supported() {
        println!("[skip] ImageAnalyzer is not supported on this Mac");
        return;
    }
    sync_analysis_off_main();
    #[cfg(feature = "async")]
    async_analysis_off_main();
    #[cfg(feature = "apple-cf")]
    in_memory_analysis_on_main();
    #[cfg(all(feature = "apple-cf", feature = "async"))]
    async_in_memory_analysis_on_main();
    println!("off-main analysis completed while the main thread ran its run loop");
}
