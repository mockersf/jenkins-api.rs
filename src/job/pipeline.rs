use failure::Error;

use client::Jenkins;
use helpers::Class;

use super::Job;
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

impl WorkflowJob {
    /// Build this job
    pub fn build(&self, jenkins_client: &Jenkins) -> Result<ShortQueueItem, Error> {
        self.builder(jenkins_client)?.send()
    }

    /// Create a `JobBuilder` to setup a build of a `Job`
    pub fn builder<'a, 'b, 'c, 'd>(
        &'a self,
        jenkins_client: &'b Jenkins,
    ) -> Result<JobBuilder<'a, 'b, 'c, 'd>, Error> {
        JobBuilder::new(self, jenkins_client)
    }
}
