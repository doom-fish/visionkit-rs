#ifndef VISIONKIT_BRIDGE_H
#define VISIONKIT_BRIDGE_H

#include <stdint.h>

void vk_string_free(char *string);
void vk_bytes_free(void *bytes);

void *vk_image_analyzer_new(void);
void vk_image_analyzer_release(void *token);
int32_t vk_image_analyzer_is_supported(void);
int32_t vk_image_analyzer_supported_text_recognition_languages_json(
    char **out_languages_json,
    char **out_error_message
);
int32_t vk_image_analyzer_analyze_image_at_path(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    void **out_analysis_token,
    char **out_error_message
);
int32_t vk_image_analyzer_analyze_ns_image_at_path(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    void **out_analysis_token,
    char **out_error_message
);
int32_t vk_image_analyzer_analyze_cg_image_at_path(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    void **out_analysis_token,
    char **out_error_message
);
int32_t vk_image_analyzer_analyze_ci_image_at_path(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    void **out_analysis_token,
    char **out_error_message
);
int32_t vk_image_analyzer_analyze_pixel_buffer_at_path(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    void **out_analysis_token,
    char **out_error_message
);

void vk_image_analysis_release(void *token);
int32_t vk_image_analysis_transcript(
    void *token,
    char **out_transcript,
    char **out_error_message
);
int32_t vk_image_analysis_has_results(
    void *token,
    uint64_t analysis_types_raw,
    int32_t *out_has_results,
    char **out_error_message
);

void *vk_live_text_content_view_new(void);
void vk_live_text_content_view_release(void *token);
int32_t vk_live_text_content_view_frame(
    void *token,
    double *out_x,
    double *out_y,
    double *out_width,
    double *out_height,
    char **out_error_message
);
int32_t vk_live_text_content_view_set_frame(
    void *token,
    double x,
    double y,
    double width,
    double height,
    char **out_error_message
);

void *vk_live_text_tracking_image_view_new(void);
void vk_live_text_tracking_image_view_release(void *token);
int32_t vk_live_text_tracking_image_view_frame(
    void *token,
    double *out_x,
    double *out_y,
    double *out_width,
    double *out_height,
    char **out_error_message
);
int32_t vk_live_text_tracking_image_view_set_frame(
    void *token,
    double x,
    double y,
    double width,
    double height,
    char **out_error_message
);
int32_t vk_live_text_tracking_image_view_set_image_at_path(
    void *token,
    const char *path,
    char **out_error_message
);
int32_t vk_live_text_tracking_image_view_image_size(
    void *token,
    int32_t *out_has_image,
    double *out_width,
    double *out_height,
    char **out_error_message
);

void *vk_live_text_interaction_delegate_new(void);
void vk_live_text_interaction_delegate_release(void *token);
int32_t vk_live_text_interaction_delegate_config_json(
    void *token,
    char **out_config_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate_set_config_json(
    void *token,
    const char *config_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate_content_view(
    void *token,
    void **out_content_view_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate_set_content_view(
    void *token,
    void *content_view_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate_recorded_events_json(
    void *token,
    char **out_events_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate_clear_recorded_events(
    void *token,
    char **out_error_message
);

void vk_live_text_subject_release(void *token);
int32_t vk_live_text_subject_bounds(
    void *token,
    double *out_x,
    double *out_y,
    double *out_width,
    double *out_height,
    char **out_error_message
);
int32_t vk_live_text_subject_png_data(
    void *token,
    void **out_bytes,
    uint64_t *out_len,
    double *out_width,
    double *out_height,
    char **out_error_message
);

void *vk_live_text_interaction_new(void);
void *vk_live_text_interaction_new_with_delegate(void *delegate_token);
void vk_live_text_interaction_release(void *token);
int32_t vk_live_text_interaction_set_analysis(
    void *token,
    void *analysis_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_track_image_at_path(
    void *token,
    const char *path,
    char **out_error_message
);
int32_t vk_live_text_interaction_delegate(
    void *token,
    void **out_delegate_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_delegate(
    void *token,
    void *delegate_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_preferred_interaction_types(
    void *token,
    uint64_t *out_types_raw,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_preferred_interaction_types(
    void *token,
    uint64_t types_raw,
    char **out_error_message
);
int32_t vk_live_text_interaction_active_interaction_types(
    void *token,
    uint64_t *out_types_raw,
    char **out_error_message
);
int32_t vk_live_text_interaction_selectable_items_highlighted(
    void *token,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_selectable_items_highlighted(
    void *token,
    int32_t value,
    char **out_error_message
);
int32_t vk_live_text_interaction_tracking_image_view(
    void *token,
    void **out_tracking_image_view_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_tracking_image_view(
    void *token,
    void *tracking_image_view_token,
    char **out_error_message
);
int32_t vk_live_text_interaction_has_active_text_selection(
    void *token,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_reset_selection(
    void *token,
    char **out_error_message
);
int32_t vk_live_text_interaction_text(
    void *token,
    char **out_text,
    char **out_error_message
);
int32_t vk_live_text_interaction_selected_text(
    void *token,
    char **out_text,
    char **out_error_message
);
int32_t vk_live_text_interaction_selected_attributed_text_json(
    void *token,
    char **out_text_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_selected_ranges_json(
    void *token,
    char **out_ranges_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_selected_ranges_json(
    void *token,
    const char *ranges_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_contents_rect(
    void *token,
    double *out_x,
    double *out_y,
    double *out_width,
    double *out_height,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_contents_rect_needs_update(
    void *token,
    char **out_error_message
);
int32_t vk_live_text_interaction_has_interactive_item_at_point(
    void *token,
    double x,
    double y,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_has_text_at_point(
    void *token,
    double x,
    double y,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_has_data_detector_at_point(
    void *token,
    double x,
    double y,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_has_supplementary_interface_at_point(
    void *token,
    double x,
    double y,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_analysis_has_text_at_point(
    void *token,
    double x,
    double y,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_live_text_button_visible(
    void *token,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_is_supplementary_interface_hidden(
    void *token,
    int32_t *out_value,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_supplementary_interface_hidden(
    void *token,
    int32_t hidden,
    int32_t animated,
    char **out_error_message
);
int32_t vk_live_text_interaction_supplementary_interface_content_insets(
    void *token,
    double *out_top,
    double *out_left,
    double *out_bottom,
    double *out_right,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_supplementary_interface_content_insets(
    void *token,
    double top,
    double left,
    double bottom,
    double right,
    char **out_error_message
);
int32_t vk_live_text_menu_tags_json(
    char **out_menu_tags_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_supplementary_interface_font_json(
    void *token,
    char **out_font_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_supplementary_interface_font_json(
    void *token,
    const char *font_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_begin_subject_analysis_if_necessary(
    void *token,
    char **out_error_message
);
int32_t vk_live_text_interaction_subjects_json(
    void *token,
    char **out_subjects_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_highlighted_subjects_json(
    void *token,
    char **out_subjects_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_set_highlighted_subjects_json(
    void *token,
    const char *subjects_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_subject_at_json(
    void *token,
    double x,
    double y,
    char **out_subject_json,
    char **out_error_message
);
int32_t vk_live_text_interaction_image_for_subjects_png_data(
    void *token,
    const char *subjects_json,
    void **out_bytes,
    uint64_t *out_len,
    double *out_width,
    double *out_height,
    char **out_error_message
);

int32_t vk_vn_document_camera_view_controller_support_json(
    char **out_support_json,
    char **out_error_message
);
int32_t vk_data_scanner_view_controller_support_json(
    char **out_support_json,
    char **out_error_message
);
int32_t vk_recognized_text_support_json(
    char **out_support_json,
    char **out_error_message
);
int32_t vk_barcode_support_json(
    char **out_support_json,
    char **out_error_message
);
int32_t vk_recognized_item_support_json(
    char **out_support_json,
    char **out_error_message
);

/* =========================================================================
 * Async thunks (feature = "async")
 *
 * Callback signature used by every async thunk:
 *   arg0  – opaque result pointer on success; for JSON-returning thunks this
 *            is a const char* cast to const void*.
 *   arg1  – error C-string on failure, NULL on success.
 *   arg2  – Rust context pointer, passed through unchanged.
 * ========================================================================= */
typedef void (*vk_async_cb)(const void *result, const char *error, void *ctx);

void vk_image_analyzer_analyze_image_async(
    void *token,
    const char *path,
    uint32_t orientation_raw,
    const char *configuration_json,
    vk_async_cb cb,
    void *ctx
);

void vk_live_text_overlay_subjects_async(
    void *token,
    vk_async_cb cb,
    void *ctx
);

void vk_live_text_overlay_subject_at_async(
    void *token,
    double point_x,
    double point_y,
    vk_async_cb cb,
    void *ctx
);

void vk_pump_main_run_loop(uint32_t milliseconds);

#endif
