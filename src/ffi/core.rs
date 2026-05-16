#![allow(missing_docs, non_camel_case_types)]

use core::ffi::c_char;

extern "C" {
    pub fn vk_string_free(s: *mut c_char);
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNAVAILABLE_ON_THIS_MACOS: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const UNAVAILABLE_ON_THIS_PLATFORM: i32 = -4;
    pub const ANALYZER_NOT_SUPPORTED: i32 = -10;
    pub const FRAMEWORK_ERROR: i32 = -20;
    pub const UNKNOWN: i32 = -99;
}
