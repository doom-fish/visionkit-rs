#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn vk_string_free(s: *mut c_char);

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

    pub fn vk_image_analysis_release(token: *mut c_void);
    pub fn vk_image_analysis_transcript(
        token: *mut c_void,
        out_transcript: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_image_analysis_has_results(
        token: *mut c_void,
        analysis_types_raw: u64,
        out_has_results: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNAVAILABLE_ON_THIS_MACOS: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const ANALYZER_NOT_SUPPORTED: i32 = -10;
    pub const FRAMEWORK_ERROR: i32 = -20;
    pub const UNKNOWN: i32 = -99;
}
