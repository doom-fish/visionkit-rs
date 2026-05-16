#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn vk_live_text_interaction_new() -> *mut c_void;
    pub fn vk_live_text_interaction_release(token: *mut c_void);
    pub fn vk_live_text_interaction_set_analysis(
        token: *mut c_void,
        analysis_token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_track_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_preferred_interaction_types(
        token: *mut c_void,
        out_types_raw: *mut u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_preferred_interaction_types(
        token: *mut c_void,
        types_raw: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_active_interaction_types(
        token: *mut c_void,
        out_types_raw: *mut u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_selectable_items_highlighted(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_selectable_items_highlighted(
        token: *mut c_void,
        value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_has_active_text_selection(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_reset_selection(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_text(
        token: *mut c_void,
        out_text: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_selected_text(
        token: *mut c_void,
        out_text: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_contents_rect(
        token: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_has_interactive_item_at_point(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_has_text_at_point(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_has_data_detector_at_point(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_has_supplementary_interface_at_point(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_analysis_has_text_at_point(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_live_text_button_visible(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_is_supplementary_interface_hidden(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_supplementary_interface_hidden(
        token: *mut c_void,
        hidden: i32,
        animated: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_supplementary_interface_content_insets(
        token: *mut c_void,
        out_top: *mut f64,
        out_left: *mut f64,
        out_bottom: *mut f64,
        out_right: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_supplementary_interface_content_insets(
        token: *mut c_void,
        top: f64,
        left: f64,
        bottom: f64,
        right: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
