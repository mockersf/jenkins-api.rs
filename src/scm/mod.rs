//! Source Control Management configuration

use serde::{self, Deserialize, Serialize};

use crate::helpers::Class;

mod browser;
pub use self::browser::*;

/// SCM merge options
#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MergeOptions {
    /// Merge strategy
    merge_strategy: String,
    /// Fast forward mode
    fast_forward_mode: String,
    /// Merge target
    merge_target: Option<String>,
    /// Remote branch
    remote_branch_name: Option<String>,
}

/// Trait implemented by specialization of SCM
pub trait SCM {}

/// A SCM
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonSCM {
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
