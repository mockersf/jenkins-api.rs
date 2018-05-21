//! Jenkins Jobs

use failure::Error;

use Jenkins;
use client::{Name, Path};
use queue::ShortQueueItem;

pub mod builder;
use self::builder::JobBuilder;

#[macro_use]
mod common;
pub use self::common::{BallColor, CommonJob, HealthReport, Job, JobName, ShortJob};
mod freestyle;
pub use self::freestyle::FreeStyleProject;
mod pipeline;
pub use self::pipeline::WorkflowJob;
mod matrix;
pub use self::matrix::{MatrixConfiguration, MatrixProject};
mod maven;
pub use self::maven::{MavenModule, MavenModuleSet};
mod external;
pub use self::external::ExternalJob;

impl Jenkins {
    /// Get a `Job` from it's `job_name`
    pub fn get_job<'a>(&self, job_name: impl Into<JobName<'a>>) -> Result<CommonJob, Error> {
        Ok(self.get(&Path::Job {
            name: Name::Name(job_name.into().0),
            configuration: None,
        })?
            .json()?)
    }

    /// Build a `Job` from it's `job_name`
    pub fn build_job<'a>(&self, job_name: impl Into<JobName<'a>>) -> Result<ShortQueueItem, Error> {
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
    pub fn poll_scm_job<'a>(&self, job_name: impl Into<JobName<'a>>) -> Result<(), Error> {
        self.post(&Path::PollSCMJob {
            name: Name::Name(job_name.into().0),
        })?;
        Ok(())
    }
}
