use core::ffi::{c_char, c_void};
use core::ops::{BitOr, BitOrAssign};
use core::ptr;
use std::path::Path;

use crate::error::VisionKitError;
use crate::ffi;
use crate::image_analysis::ImageAnalysis;
use crate::private::{error_from_status, path_to_cstring, string_from_ptr};

type BoolQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    out_value: *mut i32,
    out_error_message: *mut *mut c_char,
) -> i32;
type TypesQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    out_types_raw: *mut u64,
    out_error_message: *mut *mut c_char,
) -> i32;
type BoolSetterFn = unsafe extern "C" fn(
    token: *mut c_void,
    value: i32,
    out_error_message: *mut *mut c_char,
) -> i32;
type PointBoolQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    x: f64,
    y: f64,
    out_value: *mut i32,
    out_error_message: *mut *mut c_char,
) -> i32;
type RectQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    out_x: *mut f64,
    out_y: *mut f64,
    out_width: *mut f64,
    out_height: *mut f64,
    out_error_message: *mut *mut c_char,
) -> i32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LiveTextInteractionTypes(u64);

impl LiveTextInteractionTypes {
    pub const NONE: Self = Self(0);
    pub const AUTOMATIC: Self = Self(1);
    pub const TEXT_SELECTION: Self = Self(2);
    pub const DATA_DETECTORS: Self = Self(4);
    pub const IMAGE_SUBJECT: Self = Self(8);
    pub const VISUAL_LOOK_UP: Self = Self(16);
    pub const AUTOMATIC_TEXT_ONLY: Self = Self(32);

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

impl BitOr for LiveTextInteractionTypes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for LiveTextInteractionTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Default for LiveTextInteractionTypes {
    fn default() -> Self {
        Self::NONE
    }
}

pub struct LiveTextInteraction {
    token: *mut c_void,
}

impl Drop for LiveTextInteraction {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::live_text_interaction::vk_live_text_interaction_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl LiveTextInteraction {
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::live_text_interaction::vk_live_text_interaction_new() };
        if token.is_null() {
            return Err(VisionKitError::UnavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    pub fn set_analysis(&self, analysis: &ImageAnalysis) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_analysis(
                self.token,
                analysis.raw_token(),
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    pub fn track_image_at_path<P: AsRef<Path>>(&self, path: P) -> Result<(), VisionKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_track_image_at_path(
                self.token,
                path.as_ptr(),
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    pub fn preferred_interaction_types(&self) -> Result<LiveTextInteractionTypes, VisionKitError> {
        self.query_types(
            ffi::live_text_interaction::vk_live_text_interaction_preferred_interaction_types,
        )
    }

    pub fn set_preferred_interaction_types(
        &self,
        interaction_types: LiveTextInteractionTypes,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_preferred_interaction_types(
                self.token,
                interaction_types.bits(),
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    pub fn active_interaction_types(&self) -> Result<LiveTextInteractionTypes, VisionKitError> {
        self.query_types(
            ffi::live_text_interaction::vk_live_text_interaction_active_interaction_types,
        )
    }

    pub fn selectable_items_highlighted(&self) -> Result<bool, VisionKitError> {
        self.query_bool(
            ffi::live_text_interaction::vk_live_text_interaction_selectable_items_highlighted,
        )
    }

    pub fn set_selectable_items_highlighted(&self, value: bool) -> Result<(), VisionKitError> {
        self.set_bool(
            value,
            ffi::live_text_interaction::vk_live_text_interaction_set_selectable_items_highlighted,
        )
    }

    pub fn has_active_text_selection(&self) -> Result<bool, VisionKitError> {
        self.query_bool(
            ffi::live_text_interaction::vk_live_text_interaction_has_active_text_selection,
        )
    }

    pub fn reset_selection(&self) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_reset_selection(
                self.token,
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    pub fn text(&self) -> Result<String, VisionKitError> {
        self.query_string(ffi::live_text_interaction::vk_live_text_interaction_text)
    }

    pub fn selected_text(&self) -> Result<String, VisionKitError> {
        self.query_string(ffi::live_text_interaction::vk_live_text_interaction_selected_text)
    }

    pub fn contents_rect(&self) -> Result<Rect, VisionKitError> {
        self.query_rect(ffi::live_text_interaction::vk_live_text_interaction_contents_rect)
    }

    pub fn has_interactive_item_at_point(&self, x: f64, y: f64) -> Result<bool, VisionKitError> {
        self.query_point_bool(
            x,
            y,
            ffi::live_text_interaction::vk_live_text_interaction_has_interactive_item_at_point,
        )
    }

    pub fn has_text_at_point(&self, x: f64, y: f64) -> Result<bool, VisionKitError> {
        self.query_point_bool(
            x,
            y,
            ffi::live_text_interaction::vk_live_text_interaction_has_text_at_point,
        )
    }

    pub fn has_data_detector_at_point(&self, x: f64, y: f64) -> Result<bool, VisionKitError> {
        self.query_point_bool(
            x,
            y,
            ffi::live_text_interaction::vk_live_text_interaction_has_data_detector_at_point,
        )
    }

    pub fn has_supplementary_interface_at_point(
        &self,
        x: f64,
        y: f64,
    ) -> Result<bool, VisionKitError> {
        self.query_point_bool(
            x,
            y,
            ffi::live_text_interaction::vk_live_text_interaction_has_supplementary_interface_at_point,
        )
    }

    pub fn analysis_has_text_at_point(&self, x: f64, y: f64) -> Result<bool, VisionKitError> {
        self.query_point_bool(
            x,
            y,
            ffi::live_text_interaction::vk_live_text_interaction_analysis_has_text_at_point,
        )
    }

    pub fn live_text_button_visible(&self) -> Result<bool, VisionKitError> {
        self.query_bool(
            ffi::live_text_interaction::vk_live_text_interaction_live_text_button_visible,
        )
    }

    pub fn is_supplementary_interface_hidden(&self) -> Result<bool, VisionKitError> {
        self.query_bool(
            ffi::live_text_interaction::vk_live_text_interaction_is_supplementary_interface_hidden,
        )
    }

    pub fn set_supplementary_interface_hidden(
        &self,
        hidden: bool,
        animated: bool,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_supplementary_interface_hidden(
                self.token,
                i32::from(hidden),
                i32::from(animated),
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    pub fn supplementary_interface_content_insets(&self) -> Result<EdgeInsets, VisionKitError> {
        let rect = self.query_rect(
            ffi::live_text_interaction::vk_live_text_interaction_supplementary_interface_content_insets,
        )?;
        Ok(EdgeInsets {
            top: rect.x,
            left: rect.y,
            bottom: rect.width,
            right: rect.height,
        })
    }

    pub fn set_supplementary_interface_content_insets(
        &self,
        insets: EdgeInsets,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_supplementary_interface_content_insets(
                self.token,
                insets.top,
                insets.left,
                insets.bottom,
                insets.right,
                &mut err_msg,
            )
        };
        Self::status_to_unit(status, err_msg)
    }

    fn status_to_unit(status: i32, err_msg: *mut c_char) -> Result<(), VisionKitError> {
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn query_bool(&self, query: BoolQueryFn) -> Result<bool, VisionKitError> {
        let mut value = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { query(self.token, &mut value, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(value != 0)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn set_bool(&self, value: bool, setter: BoolSetterFn) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { setter(self.token, i32::from(value), &mut err_msg) };
        Self::status_to_unit(status, err_msg)
    }

    fn query_types(&self, query: TypesQueryFn) -> Result<LiveTextInteractionTypes, VisionKitError> {
        let mut raw = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { query(self.token, &mut raw, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(LiveTextInteractionTypes::new(raw))
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn query_string(
        &self,
        query: unsafe extern "C" fn(*mut c_void, *mut *mut c_char, *mut *mut c_char) -> i32,
    ) -> Result<String, VisionKitError> {
        let mut value: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { query(self.token, &mut value, &mut err_msg) };
        if status == ffi::status::OK {
            unsafe { string_from_ptr(value, "live text interaction string") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn query_rect(&self, query: RectQueryFn) -> Result<Rect, VisionKitError> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            query(
                self.token,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(Rect {
                x,
                y,
                width,
                height,
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn query_point_bool(
        &self,
        x: f64,
        y: f64,
        query: PointBoolQueryFn,
    ) -> Result<bool, VisionKitError> {
        let mut value = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { query(self.token, x, y, &mut value, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(value != 0)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
