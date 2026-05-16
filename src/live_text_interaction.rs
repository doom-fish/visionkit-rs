use core::ffi::{c_char, c_void};
use core::ops::{BitOr, BitOrAssign};
use core::ptr;
use std::path::Path;
use std::sync::OnceLock;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::VisionKitError;
use crate::ffi;
use crate::image_analysis::ImageAnalysis;
use crate::private::{
    error_from_status, json_cstring, parse_json_ptr, path_to_cstring, string_from_ptr,
    vec_from_buffer_ptr,
};

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
type JsonQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    out_json: *mut *mut c_char,
    out_error_message: *mut *mut c_char,
) -> i32;
type JsonSetterFn = unsafe extern "C" fn(
    token: *mut c_void,
    json: *const c_char,
    out_error_message: *mut *mut c_char,
) -> i32;
type OptionalTokenQueryFn = unsafe extern "C" fn(
    token: *mut c_void,
    out_token: *mut *mut c_void,
    out_error_message: *mut *mut c_char,
) -> i32;
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextTextRange {
    pub location: usize,
    pub length: usize,
}

impl LiveTextTextRange {
    #[must_use]
    pub const fn new(location: usize, length: usize) -> Self {
        Self { location, length }
    }

    #[must_use]
    pub const fn end(self) -> usize {
        self.location.saturating_add(self.length)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextAttributedTextAttribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextAttributedTextRun {
    pub range: LiveTextTextRange,
    pub attributes: Vec<LiveTextAttributedTextAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextAttributedText {
    pub text: String,
    pub runs: Vec<LiveTextAttributedTextRun>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LiveTextMenuTag(i64);

impl LiveTextMenuTag {
    #[must_use]
    pub const fn new(raw_value: i64) -> Self {
        Self(raw_value)
    }

    #[must_use]
    pub const fn raw_value(self) -> i64 {
        self.0
    }

    pub fn copy_image() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.copy_image))
    }

    pub fn share_image() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.share_image))
    }

    pub fn copy_subject() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.copy_subject))
    }

    pub fn share_subject() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.share_subject))
    }

    pub fn lookup_item() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.lookup_item))
    }

    pub fn recommended_app_items() -> Result<Self, VisionKitError> {
        Ok(Self(live_text_menu_tag_constants()?.recommended_app_items))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextMenuItem {
    pub title: String,
    pub tag: i64,
    pub is_separator: bool,
    pub is_enabled: bool,
    pub is_hidden: bool,
    pub state: i64,
    pub submenu: Option<Box<LiveTextMenu>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextMenu {
    pub title: String,
    pub items: Vec<LiveTextMenuItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextEventInfo {
    pub type_name: String,
    pub location_in_window: Point,
    pub modifier_flags: u64,
    pub key_code: u16,
    pub characters: Option<String>,
    pub characters_ignoring_modifiers: Option<String>,
    pub click_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextDelegateEvent {
    pub kind: String,
    pub point: Option<Point>,
    pub analysis_type_raw: Option<u64>,
    pub decision: Option<bool>,
    pub rect: Option<Rect>,
    pub event: Option<LiveTextEventInfo>,
    pub menu: Option<LiveTextMenu>,
    pub menu_item: Option<LiveTextMenuItem>,
    pub visible: Option<bool>,
    pub highlighted: Option<bool>,
    pub has_content_view: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTextFont {
    pub name: String,
    pub point_size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveTextImageData {
    pub size: Size,
    pub png_data: Vec<u8>,
}

impl LiveTextImageData {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.size.is_empty() || self.png_data.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LiveTextMenuTagConstants {
    copy_image: i64,
    share_image: i64,
    copy_subject: i64,
    share_subject: i64,
    lookup_item: i64,
    recommended_app_items: i64,
}

static LIVE_TEXT_MENU_TAGS: OnceLock<Result<LiveTextMenuTagConstants, VisionKitError>> =
    OnceLock::new();

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LiveTextInteractionDelegateConfigPayload {
    should_begin: bool,
    contents_rect: Option<Rect>,
    should_handle_key_down_event: bool,
    should_show_menu_for_event: bool,
    updated_menu: Option<LiveTextMenu>,
}

impl Default for LiveTextInteractionDelegateConfigPayload {
    fn default() -> Self {
        Self {
            should_begin: true,
            contents_rect: None,
            should_handle_key_down_event: true,
            should_show_menu_for_event: true,
            updated_menu: None,
        }
    }
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

pub struct LiveTextContentView {
    token: *mut c_void,
}

impl Drop for LiveTextContentView {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::live_text_interaction::vk_live_text_content_view_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl LiveTextContentView {
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::live_text_interaction::vk_live_text_content_view_new() };
        if token.is_null() {
            return Err(VisionKitError::Unknown(
                "failed to allocate LiveTextContentView".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    pub fn frame(&self) -> Result<Rect, VisionKitError> {
        query_rect_call("live text content view frame", |out_x, out_y, out_width, out_height, out_error_message| unsafe {
            ffi::live_text_interaction::vk_live_text_content_view_frame(
                self.token,
                out_x,
                out_y,
                out_width,
                out_height,
                out_error_message,
            )
        })
    }

    pub fn set_frame(&self, frame: Rect) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_content_view_set_frame(
                self.token,
                frame.x,
                frame.y,
                frame.width,
                frame.height,
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub(crate) fn raw_token(&self) -> *mut c_void {
        self.token
    }

    fn from_token(token: *mut c_void) -> Self {
        Self { token }
    }
}

pub struct LiveTextTrackingImageView {
    token: *mut c_void,
}

impl Drop for LiveTextTrackingImageView {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::live_text_interaction::vk_live_text_tracking_image_view_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl LiveTextTrackingImageView {
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::live_text_interaction::vk_live_text_tracking_image_view_new() };
        if token.is_null() {
            return Err(VisionKitError::Unknown(
                "failed to allocate LiveTextTrackingImageView".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    pub fn frame(&self) -> Result<Rect, VisionKitError> {
        query_rect_call(
            "live text tracking image view frame",
            |out_x, out_y, out_width, out_height, out_error_message| unsafe {
                ffi::live_text_interaction::vk_live_text_tracking_image_view_frame(
                    self.token,
                    out_x,
                    out_y,
                    out_width,
                    out_height,
                    out_error_message,
                )
            },
        )
    }

    pub fn set_frame(&self, frame: Rect) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_tracking_image_view_set_frame(
                self.token,
                frame.x,
                frame.y,
                frame.width,
                frame.height,
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn set_image_at_path<P: AsRef<Path>>(&self, path: P) -> Result<(), VisionKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_tracking_image_view_set_image_at_path(
                self.token,
                path.as_ptr(),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn image_size(&self) -> Result<Option<Size>, VisionKitError> {
        let mut has_image = 0;
        let mut width = 0.0;
        let mut height = 0.0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_tracking_image_view_image_size(
                self.token,
                &mut has_image,
                &mut width,
                &mut height,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok((has_image != 0).then_some(Size { width, height }))
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub(crate) fn raw_token(&self) -> *mut c_void {
        self.token
    }

    fn from_token(token: *mut c_void) -> Self {
        Self { token }
    }
}

pub struct LiveTextInteractionDelegate {
    token: *mut c_void,
}

impl Drop for LiveTextInteractionDelegate {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::live_text_interaction::vk_live_text_interaction_delegate_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl LiveTextInteractionDelegate {
    pub fn new() -> Result<Self, VisionKitError> {
        let token = unsafe { ffi::live_text_interaction::vk_live_text_interaction_delegate_new() };
        if token.is_null() {
            return Err(VisionKitError::UnavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    pub fn should_begin(&self) -> Result<bool, VisionKitError> {
        Ok(self.config()?.should_begin)
    }

    pub fn set_should_begin(&self, value: bool) -> Result<(), VisionKitError> {
        let mut config = self.config()?;
        config.should_begin = value;
        self.set_config(&config)
    }

    pub fn contents_rect_override(&self) -> Result<Option<Rect>, VisionKitError> {
        Ok(self.config()?.contents_rect)
    }

    pub fn set_contents_rect_override(&self, value: Option<Rect>) -> Result<(), VisionKitError> {
        let mut config = self.config()?;
        config.contents_rect = value;
        self.set_config(&config)
    }

    pub fn content_view(&self) -> Result<Option<LiveTextContentView>, VisionKitError> {
        optional_token_call(|out_token, out_error_message| unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_delegate_content_view(
                self.token,
                out_token,
                out_error_message,
            )
        })
        .map(|token| token.map(LiveTextContentView::from_token))
    }

    pub fn set_content_view(
        &self,
        value: Option<&LiveTextContentView>,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_delegate_set_content_view(
                self.token,
                value.map_or(ptr::null_mut(), LiveTextContentView::raw_token),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn should_handle_key_down_event(&self) -> Result<bool, VisionKitError> {
        Ok(self.config()?.should_handle_key_down_event)
    }

    pub fn set_should_handle_key_down_event(&self, value: bool) -> Result<(), VisionKitError> {
        let mut config = self.config()?;
        config.should_handle_key_down_event = value;
        self.set_config(&config)
    }

    pub fn should_show_menu_for_event(&self) -> Result<bool, VisionKitError> {
        Ok(self.config()?.should_show_menu_for_event)
    }

    pub fn set_should_show_menu_for_event(&self, value: bool) -> Result<(), VisionKitError> {
        let mut config = self.config()?;
        config.should_show_menu_for_event = value;
        self.set_config(&config)
    }

    pub fn updated_menu(&self) -> Result<Option<LiveTextMenu>, VisionKitError> {
        Ok(self.config()?.updated_menu)
    }

    pub fn set_updated_menu(&self, value: Option<&LiveTextMenu>) -> Result<(), VisionKitError> {
        let mut config = self.config()?;
        config.updated_menu = value.cloned();
        self.set_config(&config)
    }

    pub fn recorded_events(&self) -> Result<Vec<LiveTextDelegateEvent>, VisionKitError> {
        parse_json_call(
            |out_json, out_error_message| unsafe {
                ffi::live_text_interaction::vk_live_text_interaction_delegate_recorded_events_json(
                    self.token,
                    out_json,
                    out_error_message,
                )
            },
            "live text interaction delegate recorded events",
        )
    }

    pub fn clear_recorded_events(&self) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_delegate_clear_recorded_events(
                self.token,
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub(crate) fn raw_token(&self) -> *mut c_void {
        self.token
    }

    fn from_token(token: *mut c_void) -> Self {
        Self { token }
    }

    fn config(&self) -> Result<LiveTextInteractionDelegateConfigPayload, VisionKitError> {
        parse_json_call(
            |out_json, out_error_message| unsafe {
                ffi::live_text_interaction::vk_live_text_interaction_delegate_config_json(
                    self.token,
                    out_json,
                    out_error_message,
                )
            },
            "live text interaction delegate config",
        )
    }

    fn set_config(&self, config: &LiveTextInteractionDelegateConfigPayload) -> Result<(), VisionKitError> {
        let config_json = json_cstring(config)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_delegate_set_config_json(
                self.token,
                config_json.as_ptr(),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }
}

pub struct LiveTextSubject {
    token: *mut c_void,
}

impl Drop for LiveTextSubject {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::live_text_interaction::vk_live_text_subject_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl LiveTextSubject {
    pub fn bounds(&self) -> Result<Rect, VisionKitError> {
        query_rect_call("live text subject bounds", |out_x, out_y, out_width, out_height, out_error_message| unsafe {
            ffi::live_text_interaction::vk_live_text_subject_bounds(
                self.token,
                out_x,
                out_y,
                out_width,
                out_height,
                out_error_message,
            )
        })
    }

    pub fn image(&self) -> Result<LiveTextImageData, VisionKitError> {
        query_image_data_call(
            |out_bytes, out_len, out_width, out_height, out_error_message| unsafe {
                ffi::live_text_interaction::vk_live_text_subject_png_data(
                    self.token,
                    out_bytes,
                    out_len,
                    out_width,
                    out_height,
                    out_error_message,
                )
            },
            "live text subject image",
        )
    }

    pub(crate) fn raw_token(&self) -> *mut c_void {
        self.token
    }

    fn from_token(token: *mut c_void) -> Self {
        Self { token }
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

    pub fn with_delegate(delegate: &LiveTextInteractionDelegate) -> Result<Self, VisionKitError> {
        let token = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_new_with_delegate(delegate.raw_token())
        };
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
        status_to_unit(status, err_msg)
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
        status_to_unit(status, err_msg)
    }

    pub fn delegate(&self) -> Result<Option<LiveTextInteractionDelegate>, VisionKitError> {
        self.query_optional_token(ffi::live_text_interaction::vk_live_text_interaction_delegate)
            .map(|token| token.map(LiveTextInteractionDelegate::from_token))
    }

    pub fn set_delegate(
        &self,
        delegate: Option<&LiveTextInteractionDelegate>,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_delegate(
                self.token,
                delegate.map_or(ptr::null_mut(), LiveTextInteractionDelegate::raw_token),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
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
        status_to_unit(status, err_msg)
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

    pub fn tracking_image_view(&self) -> Result<Option<LiveTextTrackingImageView>, VisionKitError> {
        self.query_optional_token(
            ffi::live_text_interaction::vk_live_text_interaction_tracking_image_view,
        )
        .map(|token| token.map(LiveTextTrackingImageView::from_token))
    }

    pub fn set_tracking_image_view(
        &self,
        view: Option<&LiveTextTrackingImageView>,
    ) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_tracking_image_view(
                self.token,
                view.map_or(ptr::null_mut(), LiveTextTrackingImageView::raw_token),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
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
        status_to_unit(status, err_msg)
    }

    pub fn text(&self) -> Result<String, VisionKitError> {
        self.query_string(ffi::live_text_interaction::vk_live_text_interaction_text)
    }

    pub fn selected_text(&self) -> Result<String, VisionKitError> {
        self.query_string(ffi::live_text_interaction::vk_live_text_interaction_selected_text)
    }

    pub fn selected_attributed_text(&self) -> Result<LiveTextAttributedText, VisionKitError> {
        self.query_json(
            ffi::live_text_interaction::vk_live_text_interaction_selected_attributed_text_json,
            "live text interaction selected attributed text",
        )
    }

    pub fn selected_ranges(&self) -> Result<Vec<LiveTextTextRange>, VisionKitError> {
        self.query_json(
            ffi::live_text_interaction::vk_live_text_interaction_selected_ranges_json,
            "live text interaction selected ranges",
        )
    }

    pub fn set_selected_ranges(&self, ranges: &[LiveTextTextRange]) -> Result<(), VisionKitError> {
        self.set_json(
            ranges,
            ffi::live_text_interaction::vk_live_text_interaction_set_selected_ranges_json,
        )
    }

    pub fn contents_rect(&self) -> Result<Rect, VisionKitError> {
        self.query_rect(ffi::live_text_interaction::vk_live_text_interaction_contents_rect)
    }

    pub fn set_contents_rect_needs_update(&self) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_contents_rect_needs_update(
                self.token,
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
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
        self.query_bool(ffi::live_text_interaction::vk_live_text_interaction_live_text_button_visible)
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
        status_to_unit(status, err_msg)
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
        status_to_unit(status, err_msg)
    }

    pub fn supplementary_interface_font(&self) -> Result<Option<LiveTextFont>, VisionKitError> {
        self.query_json(
            ffi::live_text_interaction::vk_live_text_interaction_supplementary_interface_font_json,
            "live text interaction supplementary interface font",
        )
    }

    pub fn set_supplementary_interface_font(
        &self,
        font: Option<&LiveTextFont>,
    ) -> Result<(), VisionKitError> {
        let font_json = json_cstring(&font)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_supplementary_interface_font_json(
                self.token,
                font_json.as_ptr(),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn begin_subject_analysis_if_necessary(&self) -> Result<(), VisionKitError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_begin_subject_analysis_if_necessary(
                self.token,
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn subjects(&self) -> Result<Vec<LiveTextSubject>, VisionKitError> {
        self.query_subjects(ffi::live_text_interaction::vk_live_text_interaction_subjects_json)
    }

    pub fn highlighted_subjects(&self) -> Result<Vec<LiveTextSubject>, VisionKitError> {
        self.query_subjects(
            ffi::live_text_interaction::vk_live_text_interaction_highlighted_subjects_json,
        )
    }

    pub fn set_highlighted_subjects(
        &self,
        subjects: &[LiveTextSubject],
    ) -> Result<(), VisionKitError> {
        let subjects_json = json_cstring(&subject_tokens(subjects))?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_set_highlighted_subjects_json(
                self.token,
                subjects_json.as_ptr(),
                &mut err_msg,
            )
        };
        status_to_unit(status, err_msg)
    }

    pub fn subject_at_point(&self, x: f64, y: f64) -> Result<Option<LiveTextSubject>, VisionKitError> {
        let mut subject_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_subject_at_json(
                self.token,
                x,
                y,
                &mut subject_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            let token: Option<u64> = unsafe {
                parse_json_ptr(subject_json, "live text interaction subject lookup")
            }?;
            Ok(token.map(token_from_u64).map(LiveTextSubject::from_token))
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn image_for_subjects(
        &self,
        subjects: &[LiveTextSubject],
    ) -> Result<LiveTextImageData, VisionKitError> {
        let subjects_json = json_cstring(&subject_tokens(subjects))?;
        let mut bytes: *mut c_void = ptr::null_mut();
        let mut len = 0;
        let mut width = 0.0;
        let mut height = 0.0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::live_text_interaction::vk_live_text_interaction_image_for_subjects_png_data(
                self.token,
                subjects_json.as_ptr(),
                &mut bytes,
                &mut len,
                &mut width,
                &mut height,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(LiveTextImageData {
                size: Size { width, height },
                png_data: unsafe {
                    vec_from_buffer_ptr(
                        bytes.cast::<u8>(),
                        u64_to_usize(len, "live text interaction subject image")?,
                        "live text interaction subject image",
                    )
                }?,
            })
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
        status_to_unit(status, err_msg)
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

    fn query_json<T>(&self, query: JsonQueryFn, context: &str) -> Result<T, VisionKitError>
    where
        T: DeserializeOwned,
    {
        let mut value: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { query(self.token, &mut value, &mut err_msg) };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(value, context) }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn set_json<T>(&self, value: &T, setter: JsonSetterFn) -> Result<(), VisionKitError>
    where
        T: Serialize + ?Sized,
    {
        let json = json_cstring(value)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { setter(self.token, json.as_ptr(), &mut err_msg) };
        status_to_unit(status, err_msg)
    }

    fn query_rect(&self, query: RectQueryFn) -> Result<Rect, VisionKitError> {
        query_rect_call("live text interaction rect", |out_x, out_y, out_width, out_height, out_error_message| unsafe {
            query(
                self.token,
                out_x,
                out_y,
                out_width,
                out_height,
                out_error_message,
            )
        })
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

    fn query_optional_token(
        &self,
        query: OptionalTokenQueryFn,
    ) -> Result<Option<*mut c_void>, VisionKitError> {
        optional_token_call(|out_token, out_error_message| unsafe {
            query(self.token, out_token, out_error_message)
        })
    }

    fn query_subjects(&self, query: JsonQueryFn) -> Result<Vec<LiveTextSubject>, VisionKitError> {
        let tokens: Vec<u64> = self.query_json(query, "live text interaction subjects")?;
        Ok(tokens
            .into_iter()
            .map(token_from_u64)
            .map(LiveTextSubject::from_token)
            .collect())
    }
}

fn parse_json_call<T, F>(mut call: F, context: &str) -> Result<T, VisionKitError>
where
    T: DeserializeOwned,
    F: FnMut(*mut *mut c_char, *mut *mut c_char) -> i32,
{
    let mut json: *mut c_char = ptr::null_mut();
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = call(&mut json, &mut err_msg);
    if status == ffi::status::OK {
        unsafe { parse_json_ptr(json, context) }
    } else {
        Err(unsafe { error_from_status(status, err_msg) })
    }
}

fn optional_token_call<F>(mut call: F) -> Result<Option<*mut c_void>, VisionKitError>
where
    F: FnMut(*mut *mut c_void, *mut *mut c_char) -> i32,
{
    let mut token: *mut c_void = ptr::null_mut();
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = call(&mut token, &mut err_msg);
    if status == ffi::status::OK {
        Ok((!token.is_null()).then_some(token))
    } else {
        Err(unsafe { error_from_status(status, err_msg) })
    }
}

fn query_rect_call<F>(context: &str, mut call: F) -> Result<Rect, VisionKitError>
where
    F: FnMut(*mut f64, *mut f64, *mut f64, *mut f64, *mut *mut c_char) -> i32,
{
    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = 0.0;
    let mut height = 0.0;
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = call(&mut x, &mut y, &mut width, &mut height, &mut err_msg);
    if status == ffi::status::OK {
        Ok(Rect {
            x,
            y,
            width,
            height,
        })
    } else {
        let _ = context;
        Err(unsafe { error_from_status(status, err_msg) })
    }
}

fn query_image_data_call<F>(mut call: F, context: &str) -> Result<LiveTextImageData, VisionKitError>
where
    F: FnMut(*mut *mut c_void, *mut u64, *mut f64, *mut f64, *mut *mut c_char) -> i32,
{
    let mut bytes: *mut c_void = ptr::null_mut();
    let mut len = 0;
    let mut width = 0.0;
    let mut height = 0.0;
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = call(&mut bytes, &mut len, &mut width, &mut height, &mut err_msg);
    if status == ffi::status::OK {
        Ok(LiveTextImageData {
            size: Size { width, height },
            png_data: unsafe {
                vec_from_buffer_ptr(bytes.cast::<u8>(), u64_to_usize(len, context)?, context)
            }?,
        })
    } else {
        Err(unsafe { error_from_status(status, err_msg) })
    }
}

fn live_text_menu_tag_constants() -> Result<LiveTextMenuTagConstants, VisionKitError> {
    LIVE_TEXT_MENU_TAGS
        .get_or_init(|| {
            parse_json_call(
                |out_json, out_error_message| unsafe {
                    ffi::live_text_interaction::vk_live_text_menu_tags_json(
                        out_json,
                        out_error_message,
                    )
                },
                "live text menu tags",
            )
        })
        .clone()
}

fn status_to_unit(status: i32, err_msg: *mut c_char) -> Result<(), VisionKitError> {
    if status == ffi::status::OK {
        Ok(())
    } else {
        Err(unsafe { error_from_status(status, err_msg) })
    }
}

fn subject_tokens(subjects: &[LiveTextSubject]) -> Vec<u64> {
    subjects.iter().map(|subject| token_to_u64(subject.raw_token())).collect()
}

fn token_to_u64(token: *mut c_void) -> u64 {
    token as usize as u64
}

fn token_from_u64(token: u64) -> *mut c_void {
    usize::try_from(token).map_or(ptr::null_mut(), |value| value as *mut c_void)
}

fn u64_to_usize(value: u64, context: &str) -> Result<usize, VisionKitError> {
    usize::try_from(value).map_err(|_| {
        VisionKitError::Unknown(format!(
            "{context} length exceeded this platform's address width"
        ))
    })
}
