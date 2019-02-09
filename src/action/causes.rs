//! Types to parse the causes of a `Build`

use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::helpers::Class;

/// Trait implemented by specialization of cause
pub trait Cause {}

/// A `Cause` on a `CauseAction`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonCause {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    /// Short description of the cause
    pub short_description: String,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonCause => Cause);
impl Cause for CommonCause {}

/// Caused by a user
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserIdCause {
    /// Short description of the cause
    pub short_description: String,
    /// User ID responsible
    pub user_id: String,
    /// User name responsible
    pub user_name: String,
}
register_class!("hudson.model.Cause$UserIdCause" => UserIdCause);
impl Cause for UserIdCause {}

/// Caused remotely
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCause {
    /// Short description of the cause
    pub short_description: String,
    /// addr that triggered
    pub addr: String,
    /// Note provided when triggering the build
    pub note: Option<String>,
}
register_class!("hudson.model.Cause$RemoteCause" => RemoteCause);
impl Cause for RemoteCause {}

/// Caused by another project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpstreamCause {
    /// Short description of the cause
    pub short_description: String,
    /// `Build` number that triggered this `Build`
    pub upstream_build: u32,
    /// `Job` whose `Build` triggered this `Build`
    pub upstream_project: String,
    /// URL to the upstream `Build`
    pub upstream_url: String,
}
register_class!("hudson.model.Cause$RemoteCause" => UpstreamCause);
impl Cause for UpstreamCause {}

/// Caused by a timer
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimerTriggerCause {
    /// Short description of the cause
    pub short_description: String,
}
register_class!("hudson.triggers.TimerTrigger$TimerTriggerCause" => TimerTriggerCause);
impl Cause for TimerTriggerCause {}

/// Caused by a SCM change
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SCMTriggerCause {
    /// Short description of the cause
    pub short_description: String,
}
register_class!("hudson.triggers.SCMTrigger$SCMTriggerCause" => SCMTriggerCause);
impl Cause for SCMTriggerCause {}
