//! A user, not always a Jenkins user

use serde_json;

/// Short User that is used in list and links from other structs
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortUser {
    /// Full name of the user
    pub full_name: String,
    /// Absolute URL to the user profile
    pub absolute_url: String,
    #[serde(flatten)]
    pub(crate) other_fields: Option<serde_json::Value>,
}
