use core::ffi::c_char;
use core::ptr;

use crate::error::VisionKitError;
use crate::ffi;
use crate::private::{error_from_status, parse_area_support_info_ptr};
use crate::support::AreaSupportInfo;

/// Wraps the VisionKit barcode area.
pub struct Barcode;

impl Barcode {
    /// Returns VisionKit availability metadata for this area.
    pub fn support_info() -> Result<AreaSupportInfo, VisionKitError> {
        let mut support_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::barcode::vk_barcode_support_json(&mut support_json, &mut err_msg) };
        if status == ffi::status::OK {
            unsafe { parse_area_support_info_ptr(support_json, "Barcode support info") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Returns whether this VisionKit area is available on the current platform.
    pub fn is_available_on_current_platform() -> Result<bool, VisionKitError> {
        Ok(Self::support_info()?.available_on_current_platform)
    }
}
