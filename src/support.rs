use serde::{Deserialize, Serialize};

use crate::error::VisionKitError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaSupportInfo {
    pub area: String,
    pub current_platform: String,
    pub available_on_current_platform: bool,
    pub availability: String,
    pub reason: Option<String>,
    pub members: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl AreaSupportInfo {
    #[must_use]
    pub fn unavailable_error(&self) -> VisionKitError {
        VisionKitError::UnavailableOnThisPlatform(self.reason.clone().unwrap_or_else(|| {
            let area = &self.area;
            let current_platform = &self.current_platform;
            format!("{area} is unavailable on {current_platform}")
        }))
    }
}
