use helpers::Class;

use super::{BuildableJob, Job, SCMPollable, ShortJob};
use action::CommonAction;
use build::ShortBuild;
use property::CommonProperty;
use queue::ShortQueueItem;
use scm::CommonSCM;

use super::{BallColor, HealthReport, JobBuilder};

job_build_with_common_fields_and_impl!(/// A build flow job
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildFlowJob {
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
});
register_class!("com.cloudbees.plugins.flow.BuildFlow" => BuildFlowJob );

impl BuildableJob for BuildFlowJob {}
impl SCMPollable for BuildFlowJob {}
