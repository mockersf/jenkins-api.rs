use helpers::Class;

use super::{BuildableJob, Job};
use action::CommonAction;
use build::ShortBuild;
use property::CommonProperty;
use queue::ShortQueueItem;

use super::{BallColor, HealthReport, JobBuilder};

job_build_with_common_fields_and_impl!(/// A pipeline project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowJob {
    /// Description of the job
    pub description: String,
    /// Is concurrent build enabled for the job?
    pub concurrent_build: bool,
});
register_class!("org.jenkinsci.plugins.workflow.job.WorkflowJob" => WorkflowJob);

impl BuildableJob for WorkflowJob {}
