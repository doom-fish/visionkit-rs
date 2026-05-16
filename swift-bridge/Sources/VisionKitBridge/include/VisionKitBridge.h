#ifndef VISIONKIT_BRIDGE_H
#define VISIONKIT_BRIDGE_H

#include <stdint.h>

void vk_string_free(char *string);

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

void *vk_live_text_interaction_new(void);
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
int32_t vk_live_text_interaction_contents_rect(
    void *token,
    double *out_x,
    double *out_y,
    double *out_width,
    double *out_height,
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

#endif
