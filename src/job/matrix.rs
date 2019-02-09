use serde::Deserialize;

use crate::helpers::Class;

use super::{BuildableJob, Job, SCMPollable, ShortJob};
use crate::action::CommonAction;
use crate::build::ShortBuild;
use crate::property::CommonProperty;
use crate::queue::ShortQueueItem;
use crate::scm::CommonSCM;

use crate::build::{MatrixBuild, MatrixRun};

use super::{BallColor, HealthReport};

job_build_with_common_fields_and_impl!(
    /// A matrix project
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MatrixProject<BuildType = MatrixBuild> {
        /// Description of the job
        pub description: String,
        /// Is concurrent build enabled for the job?
        pub concurrent_build: bool,
        /// SCM configured for the job
        pub scm: CommonSCM,
        /// Configurations for the job
        pub active_configurations: Vec<ShortJob<MatrixConfiguration>>,
        /// List of the upstream projects
        pub upstream_projects: Vec<ShortJob>,
        /// List of the downstream projects
        pub downstream_projects: Vec<ShortJob>,
        /// Label expression
        pub label_expression: Option<String>,
    }
);
register_class!("hudson.matrix.MatrixProject" => MatrixProject);

impl BuildableJob for MatrixProject {}
impl SCMPollable for MatrixProject {}

job_build_with_common_fields_and_impl!(
    /// A matrix configuration
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MatrixConfiguration<BuildType = MatrixRun> {
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
    }
);
register_class!("hudson.matrix.MatrixConfiguration" => MatrixConfiguration);

impl MatrixConfiguration {}
