use helpers::Class;

use super::{BuildableJob, Job, SCMPollable, ShortJob};
use action::CommonAction;
use build::{MavenBuild, MavenModuleSetBuild, ShortBuild};
use property::CommonProperty;
use queue::ShortQueueItem;
use scm::CommonSCM;

use super::{BallColor, HealthReport};

job_build_with_common_fields_and_impl!(/// A maven project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenModuleSet<BuildType = MavenModuleSetBuild> {
    /// Description of the job
    pub description: String,
    /// Is concurrent build enabled for the job?
    pub concurrent_build: bool,
    /// SCM configured for the job
    pub scm: CommonSCM,
    /// List of modules
    pub modules: Vec<ShortJob>,
    /// List of the upstream projects
    pub upstream_projects: Vec<ShortJob>,
    /// List of the downstream projects
    pub downstream_projects: Vec<ShortJob>,
    /// Label expression
    pub label_expression: Option<String>,
});
register_class!("hudson.maven.MavenModuleSet" => MavenModuleSet);

impl BuildableJob for MavenModuleSet {}
impl SCMPollable for MavenModuleSet {}

job_build_with_common_fields_and_impl!(/// A maven module
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenModule<BuildType = MavenBuild> {
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
register_class!("hudson.maven.MavenModule" => MavenModule);

impl MavenModule {}
