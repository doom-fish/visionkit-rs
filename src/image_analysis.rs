use core::ffi::{c_char, c_void};
use core::ptr;

use crate::error::VisionKitError;
use crate::ffi;
use crate::image_analyzer::ImageAnalysisTypes;
use crate::private::{error_from_status, string_from_ptr};

pub struct ImageAnalysis {
    pub(crate) token: *mut c_void,
}

impl Drop for ImageAnalysis {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::image_analysis::vk_image_analysis_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl ImageAnalysis {
    pub(crate) fn from_token(token: *mut c_void) -> Self {
        Self { token }
    }

    pub(crate) fn raw_token(&self) -> *mut c_void {
        self.token
    }

    pub fn transcript(&self) -> Result<String, VisionKitError> {
        let mut transcript: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::image_analysis::vk_image_analysis_transcript(
                self.token,
                &mut transcript,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { string_from_ptr(transcript, "image analysis transcript") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn has_results(&self, analysis_types: ImageAnalysisTypes) -> Result<bool, VisionKitError> {
        let mut has_results = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::image_analysis::vk_image_analysis_has_results(
                self.token,
                analysis_types.bits(),
                &mut has_results,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(has_results != 0)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
