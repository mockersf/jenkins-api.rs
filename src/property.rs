//! Properties of an object (Build, Job, ...)

use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::helpers::Class;

/// Trait implemented by specialization of property
pub trait Property {}

/// A Jenkins `Property`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonProperty {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonProperty => Property);
impl Property for CommonProperty {}

/// Job is a GitHub project
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubProjectProperty {}
register_class!("com.coravy.hudson.plugins.github.GithubProjectProperty" => GithubProjectProperty);
impl Property for GithubProjectProperty {}

/// Job is limited in number of concurrent builds
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitBranchProperty {}
register_class!("jenkins.branch.RateLimitBranchProperty$JobPropertyImpl" => RateLimitBranchProperty);
impl Property for RateLimitBranchProperty {}

/// Old builds of job are discarded
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuildDiscarderProperty {}
register_class!("jenkins.model.BuildDiscarderProperty" => BuildDiscarderProperty);
impl Property for BuildDiscarderProperty {}
