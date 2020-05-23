//! A user, not always a Jenkins user

use serde::{Deserialize, Serialize};

/// Short User that is used in list and links from other structs
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortUser {
    /// Full name of the user
    pub full_name: String,
    /// Absolute URL to the user profile
    pub absolute_url: String,

    #[cfg(not(feature = "extra-fields-visibility"))]
    #[serde(flatten)]
    pub(crate) extra_fields: Option<serde_json::Value>,
    #[cfg(feature = "extra-fields-visibility")]
    /// Extra fields not parsed for a common object
    #[serde(flatten)]
    pub extra_fields: Option<serde_json::Value>,
}
