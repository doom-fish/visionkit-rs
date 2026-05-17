#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn vk_live_text_content_view_new() -> *mut c_void;
    pub fn vk_live_text_content_view_release(token: *mut c_void);
    pub fn vk_live_text_content_view_frame(
        token: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_content_view_set_frame(
        token: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn vk_live_text_tracking_image_view_new() -> *mut c_void;
    pub fn vk_live_text_tracking_image_view_release(token: *mut c_void);
    pub fn vk_live_text_tracking_image_view_frame(
        token: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_tracking_image_view_set_frame(
        token: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_tracking_image_view_set_image_at_path(
        token: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_tracking_image_view_image_size(
        token: *mut c_void,
        out_has_image: *mut i32,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn vk_live_text_interaction_delegate_new() -> *mut c_void;
    pub fn vk_live_text_interaction_delegate_release(token: *mut c_void);
    pub fn vk_live_text_interaction_delegate_config_json(
        token: *mut c_void,
        out_config_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_delegate_set_config_json(
        token: *mut c_void,
        config_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_delegate_content_view(
        token: *mut c_void,
        out_content_view_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_delegate_set_content_view(
        token: *mut c_void,
        content_view_token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_delegate_recorded_events_json(
        token: *mut c_void,
        out_events_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_delegate_clear_recorded_events(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn vk_live_text_subject_release(token: *mut c_void);
    pub fn vk_live_text_subject_bounds(
        token: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_subject_png_data(
        token: *mut c_void,
        out_bytes: *mut *mut c_void,
        out_len: *mut u64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn vk_live_text_interaction_new() -> *mut c_void;
    pub fn vk_live_text_interaction_new_with_delegate(delegate_token: *mut c_void) -> *mut c_void;
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
    pub fn vk_live_text_interaction_delegate(
        token: *mut c_void,
        out_delegate_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_delegate(
        token: *mut c_void,
        delegate_token: *mut c_void,
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
    pub fn vk_live_text_interaction_tracking_image_view(
        token: *mut c_void,
        out_tracking_image_view_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_tracking_image_view(
        token: *mut c_void,
        tracking_image_view_token: *mut c_void,
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
    pub fn vk_live_text_interaction_selected_attributed_text_json(
        token: *mut c_void,
        out_text_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_selected_ranges_json(
        token: *mut c_void,
        out_ranges_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_selected_ranges_json(
        token: *mut c_void,
        ranges_json: *const c_char,
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
    pub fn vk_live_text_interaction_set_contents_rect_needs_update(
        token: *mut c_void,
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
    pub fn vk_live_text_menu_tags_json(
        out_menu_tags_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_supplementary_interface_font_json(
        token: *mut c_void,
        out_font_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_supplementary_interface_font_json(
        token: *mut c_void,
        font_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_begin_subject_analysis_if_necessary(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_subjects_json(
        token: *mut c_void,
        out_subjects_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_highlighted_subjects_json(
        token: *mut c_void,
        out_subjects_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_set_highlighted_subjects_json(
        token: *mut c_void,
        subjects_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_subject_at_json(
        token: *mut c_void,
        x: f64,
        y: f64,
        out_subject_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn vk_live_text_interaction_image_for_subjects_png_data(
        token: *mut c_void,
        subjects_json: *const c_char,
        out_bytes: *mut *mut c_void,
        out_len: *mut u64,
        out_width: *mut f64,
        out_height: *mut f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

#[cfg(feature = "async")]
extern "C" {
    /// True-async thunk for `ImageAnalysisOverlayView.subjects` (macOS).
    ///
    /// Fires `cb(json_ptr, nil, ctx)` on success, where the result pointer is a
    /// JSON-encoded array of `{"x":…,"y":…,"width":…,"height":…}` objects cast
    /// to `*const c_void`.
    pub fn vk_live_text_overlay_subjects_async(
        token: *mut c_void,
        cb: crate::ffi::image_analyzer::VkAsyncCb,
        ctx: *mut c_void,
    );

    /// True-async thunk for `ImageAnalysisOverlayView.subject(at:)` (macOS).
    ///
    /// Fires `cb(json_ptr, nil, ctx)` on success, where the result pointer is
    /// either `"null"` (no subject) or a JSON-encoded bounds object; both cast
    /// to `*const c_void`.
    pub fn vk_live_text_overlay_subject_at_async(
        token: *mut c_void,
        point_x: f64,
        point_y: f64,
        cb: crate::ffi::image_analyzer::VkAsyncCb,
        ctx: *mut c_void,
    );
}
