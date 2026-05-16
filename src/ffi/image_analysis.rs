#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
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
