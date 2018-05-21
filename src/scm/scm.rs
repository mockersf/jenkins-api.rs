use serde;
use serde_json;

use helpers::Class;

use scm::{CommonBrowser, MergeOptions};

/// Trait implemented by specialization of SCM
pub trait SCM {}

/// A SCM
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonSCM {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonSCM => SCM);
impl SCM for CommonSCM {}

/// No SCM
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NullSCM {
    /// Browser
    pub browser: Option<CommonBrowser>,
}
register_class!("hudson.scm.NullSCM" =>  NullSCM);
impl SCM for NullSCM {}

/// Git SCM
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitSCM {
    /// Browser
    pub browser: Option<CommonBrowser>,
    /// Merge options
    pub merge_options: MergeOptions,
}
register_class!("hudson.plugins.git.GitSCM" =>  GitSCM);
impl SCM for GitSCM {}
