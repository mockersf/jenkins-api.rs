use failure::Error;

use client::{self, Jenkins, Path};
use helpers::Class;

use super::{Job, ShortJob};
use action::CommonAction;
use build::ShortBuild;
use property::CommonProperty;
use queue::ShortQueueItem;
use scm::CommonSCM;

use super::{BallColor, HealthReport, JobBuilder};

job_build_with_common_fields_and_impl!(/// A matrix project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatrixProject {
    /// Description of the job
    pub description: String,
    /// Is concurrent build enabled for the job?
    pub concurrent_build: bool,
    /// SCM configured for the job
    pub scm: CommonSCM,
    /// Configurations for the job
    pub active_configurations: Vec<ShortJob>,
    /// List of the upstream projects
    pub upstream_projects: Vec<ShortJob>,
    /// List of the downstream projects
    pub downstream_projects: Vec<ShortJob>,
    /// Label expression
    pub label_expression: Option<String>,
});
register_class!("hudson.matrix.MatrixProject" => MatrixProject);

impl MatrixProject {
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

    /// Poll configured SCM for changes
    pub fn poll_scm(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::PollSCMJob { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }
}

job_build_with_common_fields_and_impl!(/// A matrix configuration
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatrixConfiguration {
    /// Description of the job
    pub description: Option<String>,
    /// Is concurrent build enabled for the job?
    pub concurrent_build: bool,
    /// SCM configured for the job
    pub scm: CommonSCM,
    /// List of the upstream projects
    pub upstream_projects: Vec<ShortJob>,
    /// List of the downstream projects
    pub downstream_projects: Vec<ShortJob>,
    /// Label expression
    pub label_expression: Option<String>,
});
register_class!("hudson.matrix.MatrixConfiguration" => MatrixConfiguration);

impl MatrixConfiguration {}
