use core::ffi::{c_char, c_void};
use core::ops::{BitOr, BitOrAssign};
use core::ptr;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::VisionKitError;
use crate::ffi;
use crate::private::{
    error_from_status, json_cstring, parse_json_ptr, path_to_cstring, string_from_ptr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImageAnalysisTypes(u64);

impl ImageAnalysisTypes {
    pub const NONE: Self = Self(0);
    pub const TEXT: Self = Self(1);
    pub const MACHINE_READABLE_CODE: Self = Self(2);
    pub const VISUAL_LOOK_UP: Self = Self(4);
    pub const ALL: Self = Self(Self::TEXT.0 | Self::MACHINE_READABLE_CODE.0 | Self::VISUAL_LOOK_UP.0);

    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for ImageAnalysisTypes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ImageAnalysisTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Default for ImageAnalysisTypes {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageOrientation {
    #[default]
    Up,
    UpMirrored,
    Down,
    DownMirrored,
    LeftMirrored,
    Right,
    RightMirrored,
    Left,
}

impl ImageOrientation {
    #[must_use]
    pub const fn raw_value(self) -> u32 {
        match self {
            Self::Up => 1,
            Self::UpMirrored => 2,
            Self::Down => 3,
            Self::DownMirrored => 4,
            Self::LeftMirrored => 5,
            Self::Right => 6,
            Self::RightMirrored => 7,
            Self::Left => 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAnalyzerConfiguration {
    analysis_types: ImageAnalysisTypes,
    locales: Vec<String>,
}

impl ImageAnalyzerConfiguration {
    #[must_use]
    pub fn new(analysis_types: ImageAnalysisTypes) -> Self {
        Self {
            analysis_types,
            locales: Vec::new(),
        }
    }

    #[must_use]
    pub fn analysis_types(&self) -> ImageAnalysisTypes {
        self.analysis_types
    }

    #[must_use]
    pub fn locales(&self) -> &[String] {
        &self.locales
    }

    #[must_use]
    pub fn with_locales<I, S>(mut self, locales: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.locales = locales.into_iter().map(Into::into).collect();
        self
    }
}

pub struct ImageAnalysis {
    token: *mut c_void,
}

impl Drop for ImageAnalysis {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::vk_image_analysis_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl ImageAnalysis {
    pub fn transcript(&self) -> Result<String, VisionKitError> {
        let mut transcript: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::vk_image_analysis_transcript(self.token, &mut transcript, &mut err_msg)
        };
        if status == ffi::status::OK {
            unsafe { string_from_ptr(transcript, "analysis transcript") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn has_results(
        &self,
        analysis_types: ImageAnalysisTypes,
    ) -> Result<bool, VisionKitError> {
        let mut has_results = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::vk_image_analysis_has_results(
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

pub struct ImageAnalyzer {
    token: *mut c_void,
}

impl Drop for ImageAnalyzer {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::vk_image_analyzer_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl ImageAnalyzer {
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::vk_image_analyzer_new() };
        if token.is_null() {
            return Err(VisionKitError::UnavailableOnThisMacOS(
                "ImageAnalyzer requires macOS 13+".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    #[must_use]
    pub fn is_supported() -> bool {
        unsafe { ffi::vk_image_analyzer_is_supported() != 0 }
    }

    pub fn supported_text_recognition_languages() -> Result<Vec<String>, VisionKitError> {
        let mut languages_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::vk_image_analyzer_supported_text_recognition_languages_json(
                &mut languages_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            let mut languages: Vec<String> =
                unsafe { parse_json_ptr(languages_json, "supported text recognition languages") }?;
            languages.sort();
            Ok(languages)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn analyze_image_at_path<P: AsRef<Path>>(
        &self,
        path: P,
        orientation: ImageOrientation,
        configuration: &ImageAnalyzerConfiguration,
    ) -> Result<ImageAnalysis, VisionKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let configuration_json = json_cstring(configuration)?;
        let mut analysis_token: *mut c_void = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::vk_image_analyzer_analyze_image_at_path(
                self.token,
                path.as_ptr(),
                orientation.raw_value(),
                configuration_json.as_ptr(),
                &mut analysis_token,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            if analysis_token.is_null() {
                return Err(VisionKitError::Unknown(
                    "Swift bridge returned an empty image analysis token".to_owned(),
                ));
            }
            Ok(ImageAnalysis {
                token: analysis_token,
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
