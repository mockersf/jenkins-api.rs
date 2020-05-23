use serde::Deserialize;

use crate::helpers::Class;

use super::{BuildableJob, Job, SCMPollable, ShortJob};
use crate::action::CommonAction;
use crate::build::{FreeStyleBuild, ShortBuild};
use crate::property::CommonProperty;
use crate::queue::ShortQueueItem;
use crate::scm::CommonSCM;

use super::{BallColor, HealthReport};

job_buildable_with_common_fields_and_impl!(
    /// A free style project
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct FreeStyleProject<BuildType = FreeStyleBuild> {
        /// Description of the job
        pub description: String,
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
register_class!("hudson.model.FreeStyleProject" => FreeStyleProject);

impl BuildableJob for FreeStyleProject {}
impl SCMPollable for FreeStyleProject {}
