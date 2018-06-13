//! Jenkins Builds

use failure::Error;

use Jenkins;
// use action::Action;
use client::{Name, Path};
use job::JobName;

#[macro_use]
mod common;
pub use self::common::{Artifact, Build, BuildNumber, BuildStatus, CommonBuild, ShortBuild};
mod freestyle;
pub use self::freestyle::FreeStyleBuild;
mod pipeline;
pub use self::pipeline::WorkflowRun;
mod matrix;
pub use self::matrix::{MatrixBuild, MatrixRun};
mod maven;
pub use self::maven::{MavenBuild, MavenModuleSetBuild};

impl Jenkins {
    /// Get a build from a `job_name` and `build_number`
    pub fn get_build<'a, J, B>(&self, job_name: J, build_number: B) -> Result<CommonBuild, Error>
    where
        J: Into<JobName<'a>>,
        B: Into<BuildNumber>,
    {
        Ok(self.get(&Path::Build {
            job_name: Name::Name(job_name.into().0),
            number: build_number.into(),
            configuration: None,
        })?
            .json()?)
    }
}
