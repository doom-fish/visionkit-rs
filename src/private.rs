use core::ffi::{c_char, CStr};
use std::ffi::CString;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::VisionKitError;
use crate::ffi;
use crate::support::AreaSupportInfo;

pub fn to_cstring(value: &str) -> Result<CString, VisionKitError> {
    CString::new(value).map_err(|_| {
        VisionKitError::InvalidArgument("string contained an interior NUL byte".to_owned())
    })
}

pub fn path_to_cstring(path: &Path) -> Result<CString, VisionKitError> {
    let path = path.as_os_str().to_string_lossy();
    to_cstring(&path)
}

pub fn json_cstring<T: Serialize + ?Sized>(value: &T) -> Result<CString, VisionKitError> {
    let json = serde_json::to_string(value).map_err(|error| {
        VisionKitError::Unknown(format!("failed to encode JSON payload: {error}"))
    })?;
    to_cstring(&json)
}

pub unsafe fn take_optional_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::vk_string_free(ptr);
    Some(string)
}

pub unsafe fn string_from_ptr(ptr: *mut c_char, context: &str) -> Result<String, VisionKitError> {
    take_optional_string(ptr).ok_or_else(|| {
        VisionKitError::Unknown(format!("missing {context} response from Swift bridge"))
    })
}

pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, VisionKitError> {
    let json = string_from_ptr(ptr, context)?;
    serde_json::from_str(&json).map_err(|error| {
        VisionKitError::Unknown(format!("failed to decode {context} JSON payload: {error}"))
    })
}

pub unsafe fn parse_area_support_info_ptr(
    ptr: *mut c_char,
    context: &str,
) -> Result<AreaSupportInfo, VisionKitError> {
    parse_json_ptr(ptr, context)
}

pub unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> VisionKitError {
    let message = take_optional_string(err_msg)
        .unwrap_or_else(|| format!("Swift bridge call failed with status code {status}"));
    match status {
        ffi::status::INVALID_ARGUMENT => VisionKitError::InvalidArgument(message),
        ffi::status::UNAVAILABLE_ON_THIS_MACOS => VisionKitError::UnavailableOnThisMacOS(message),
        ffi::status::UNAVAILABLE_ON_THIS_PLATFORM => {
            VisionKitError::UnavailableOnThisPlatform(message)
        }
        ffi::status::TIMED_OUT => VisionKitError::TimedOut(message),
        ffi::status::ANALYZER_NOT_SUPPORTED => VisionKitError::AnalyzerNotSupported(message),
        ffi::status::FRAMEWORK_ERROR => VisionKitError::Framework(message),
        _ => VisionKitError::Unknown(message),
    }
}
