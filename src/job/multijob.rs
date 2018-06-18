use helpers::Class;

use super::{BuildableJob, Job, SCMPollable, ShortJob};
use action::CommonAction;
use build::{MultiJobBuild, ShortBuild};
use property::CommonProperty;
use queue::ShortQueueItem;
use scm::CommonSCM;

use super::{BallColor, HealthReport};

job_build_with_common_fields_and_impl!(/// A MultiJob Project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiJobProject<BuildType = MultiJobBuild> {
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
register_class!("com.tikal.jenkins.plugins.multijob.MultiJobProject" => MultiJobProject);

impl BuildableJob for MultiJobProject {}
impl SCMPollable for MultiJobProject {}
