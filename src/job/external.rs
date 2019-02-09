use serde::Deserialize;

use crate::helpers::Class;

use super::Job;
use crate::action::CommonAction;
use crate::build::{CommonBuild, ShortBuild};
use crate::property::CommonProperty;
use crate::queue::ShortQueueItem;

use super::{BallColor, HealthReport};

job_build_with_common_fields_and_impl!(
    /// An external job
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ExternalJob {}
);
register_class!("hudson.model.ExternalJob" => ExternalJob);

impl ExternalJob {}
