use helpers::Class;

use super::Job;
use action::CommonAction;
use build::{CommonBuild, ShortBuild};
use property::CommonProperty;
use queue::ShortQueueItem;

use super::{BallColor, HealthReport};

job_build_with_common_fields_and_impl!(
    /// An external job
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ExternalJob {}
);
register_class!("hudson.model.ExternalJob" => ExternalJob);

impl ExternalJob {}
