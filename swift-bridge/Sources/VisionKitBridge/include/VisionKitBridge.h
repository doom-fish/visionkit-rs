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

#endif
