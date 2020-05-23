use serde::{self, Deserialize, Serialize};

use crate::helpers::Class;

/// Trait implemented by specialization of Browser
pub trait Browser {}

/// A Browser
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonBrowser {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[cfg(not(feature = "extra-fields-visibility"))]
    #[serde(flatten)]
    extra_fields: serde_json::Value,
    #[cfg(feature = "extra-fields-visibility")]
    /// Extra fields not parsed for a common object
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}
specialize!(CommonBrowser => Browser);
impl Browser for CommonBrowser {}

/// Github web browser
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubWeb {}
register_class!("hudson.plugins.git.browser.GithubWeb" =>  GithubWeb);
impl Browser for GithubWeb {}
