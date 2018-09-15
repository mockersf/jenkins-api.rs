//! Jenkins Jobs

use failure::Error;

use client_internals::{Name, Path};
use queue::ShortQueueItem;
use Jenkins;

pub mod builder;
use self::builder::JobBuilder;

#[macro_use]
mod common;
pub use self::common::{
    BallColor, BuildableJob, CommonJob, HealthReport, Job, JobName, SCMPollable, ShortJob,
};
mod flow;
pub use self::flow::BuildFlowJob;
mod freestyle;
pub use self::freestyle::FreeStyleProject;
mod pipeline;
pub use self::pipeline::WorkflowJob;
mod matrix;
pub use self::matrix::{MatrixConfiguration, MatrixProject};
mod maven;
pub use self::maven::{MavenModule, MavenModuleSet};
mod multijob;
pub use self::multijob::MultiJobProject;
mod external;
pub use self::external::ExternalJob;

impl Jenkins {
    /// Get a `Job` from it's `job_name`
    pub fn get_job<'a, J>(&self, job_name: J) -> Result<CommonJob, Error>
    where
        J: Into<JobName<'a>>,
    {
        Ok(self
            .get(&Path::Job {
                name: Name::Name(job_name.into().0),
                configuration: None,
            })?.json()?)
        // self.get_job_as(job_name, None)
    }

    /// Build a `Job` from it's `job_name`
    pub fn build_job<'a, J>(&self, job_name: J) -> Result<ShortQueueItem, Error>
    where
        J: Into<JobName<'a>>,
    {
        JobBuilder::new_from_job_name(job_name.into().0, self)?.send()
    }

    /// Create a `JobBuilder` to setup a build of a `Job` from it's `job_name`
    pub fn job_builder<'a, 'b, 'c, 'd>(
        &'b self,
        job_name: &'a str,
    ) -> Result<JobBuilder<'a, 'b, 'c, 'd>, Error> {
        JobBuilder::new_from_job_name(job_name, self)
    }

    /// Poll SCM of a `Job` from it's `job_name`
    pub fn poll_scm_job<'a, J>(&self, job_name: J) -> Result<(), Error>
    where
        J: Into<JobName<'a>>,
    {
        let _ = self.post(&Path::PollSCMJob {
            name: Name::Name(job_name.into().0),
        })?;
        Ok(())
    }
}
