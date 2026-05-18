use serde::{Deserialize, Serialize};

use crate::error::VisionKitError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents VisionKit availability information for an API area.
pub struct AreaSupportInfo {
    /// Stores the VisionKit area value.
    pub area: String,
    /// Stores the VisionKit current platform value.
    pub current_platform: String,
    /// Indicates whether VisionKit reports this area as available on the current platform.
    pub available_on_current_platform: bool,
    /// Stores the VisionKit availability value.
    pub availability: String,
    /// Stores the VisionKit reason value.
    pub reason: Option<String>,
    /// Stores the VisionKit members value.
    pub members: Vec<String>,
    #[serde(default)]
    /// Stores the VisionKit notes value.
    pub notes: Vec<String>,
}

impl AreaSupportInfo {
    #[must_use]
    /// Builds the VisionKit availability error for this unsupported area.
    pub fn unavailable_error(&self) -> VisionKitError {
        VisionKitError::UnavailableOnThisPlatform(self.reason.clone().unwrap_or_else(|| {
            let area = &self.area;
            let current_platform = &self.current_platform;
            format!("{area} is unavailable on {current_platform}")
        }))
    }
}
