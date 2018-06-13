use serde;
use serde_json;

use helpers::Class;

/// Trait implemented by specialization of Browser
pub trait Browser {}

/// A Browser
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonBrowser {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonBrowser => Browser);
impl Browser for CommonBrowser {}

/// Github web browser
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubWeb {}
register_class!("hudson.plugins.git.browser.GithubWeb" =>  GithubWeb);
impl Browser for GithubWeb {}
