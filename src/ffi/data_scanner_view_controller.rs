#![allow(missing_docs, non_camel_case_types)]

use core::ffi::c_char;

extern "C" {
    pub fn vk_data_scanner_view_controller_support_json(
        out_support_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
