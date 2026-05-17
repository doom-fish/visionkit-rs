#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn vk_image_analyzer_new() -> *mut c_void;
    pub fn vk_image_analyzer_release(token: *mut c_void);
    pub fn vk_image_analyzer_is_supported() -> i32;
    pub fn vk_image_analyzer_supported_text_recognition_languages_json(
        out_languages_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analyzer_analyze_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        out_analysis_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analyzer_analyze_ns_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        out_analysis_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analyzer_analyze_cg_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        out_analysis_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analyzer_analyze_ci_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        out_analysis_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analyzer_analyze_pixel_buffer_at_path(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        out_analysis_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

/// Async C callback type: `(result: *const c_void, error: *const i8, ctx: *mut c_void) -> ()`
pub type VkAsyncCb =
    unsafe extern "C" fn(result: *const c_void, error: *const i8, ctx: *mut c_void);

#[cfg(feature = "async")]
extern "C" {
    /// True-async thunk for `ImageAnalyzer.analyze(imageAt:orientation:configuration:)`.
    /// Fires `cb(retained VKImageAnalysisBox ptr, nil, ctx)` on success.
    pub fn vk_image_analyzer_analyze_image_async(
        token: *mut c_void,
        path: *const c_char,
        orientation_raw: u32,
        configuration_json: *const c_char,
        cb: VkAsyncCb,
        ctx: *mut c_void,
    );

    /// Pump the Obj-C main run loop for up to `milliseconds` ms.
    /// Must be called from the main thread; no-ops on non-main threads.
    pub fn vk_pump_main_run_loop(milliseconds: u32);
}
