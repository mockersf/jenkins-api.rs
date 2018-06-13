//! Types to parse the actions that triggered a `Build`

use serde;
use serde_json;

use helpers::Class;

pub mod causes;
pub mod git;
pub mod maven;
pub mod parameters;
pub mod pipeline;

/// Trait implemented by specialization of Action
pub trait Action {}

/// A node of a pipeline
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonAction {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonAction => Action);
impl Action for CommonAction {}

/// An action holding parameters
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParametersAction {
    /// The list of parameters
    pub parameters: Vec<parameters::CommonParameter>,
}
register_class!("hudson.model.ParametersAction" => ParametersAction);
impl Action for ParametersAction {}

/// An action listing causes
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CauseAction {
    /// The list of causes
    pub causes: Vec<causes::CommonCause>,
}
register_class!("hudson.model.CauseAction" => CauseAction);
impl Action for CauseAction {}

/// An action describing a Git change
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitBuildData {
    /// Name of the SCM
    pub scm_name: String,
    /// Last revision that was built
    pub last_built_revision: git::Revision,
    /// URLs to the SCM
    pub remote_urls: Vec<String>,
    /// Builds and their branches
    pub builds_by_branch_name: git::BuildsByBranch,
}
register_class!("hudson.plugins.git.util.BuildData" => GitBuildData);
impl Action for GitBuildData {}

/// An action for a git tag
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitTagAction {}
register_class!("hudson.plugins.git.GitTagAction" => GitTagAction);
impl Action for GitTagAction {}

/// An action for a repo tag
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepoTagAction {}
register_class!("hudson.plugins.repo.TagAction" => RepoTagAction);
impl Action for RepoTagAction {}

/// An action on time in queue
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeInQueueAction {}
register_class!("jenkins.metrics.impl.TimeInQueueAction" => TimeInQueueAction);
impl Action for TimeInQueueAction {}

/// An action from pipelines
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvActionImpl {}
register_class!("org.jenkinsci.plugins.workflow.cps.EnvActionImpl" => EnvActionImpl);
impl Action for EnvActionImpl {}

/// An action from pipelines
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowGraphAction {
    /// Nodes of the pipeline
    pub nodes: Vec<pipeline::CommonPipelineNode>,
}
register_class!("org.jenkinsci.plugins.workflow.job.views.FlowGraphAction" => FlowGraphAction);
impl Action for FlowGraphAction {}

/// An action with maven artifacts
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactRecord {
    /// URL to the artifacts
    pub url: String,
    /// List of the artifacts
    pub attached_artifacts: Vec<maven::Artifact>,
    /// Main artifact
    pub main_artifact: maven::Artifact,
    /// Parent build
    pub parent: ::build::ShortBuild,
    /// POM artifact
    pub pom_artifact: maven::Artifact,
}
register_class!("hudson.maven.reporters.MavenArtifactRecord" => MavenArtifactRecord);
impl Action for MavenArtifactRecord {}

/// An action with maven artifacts
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenAggregatedArtifactRecord {
    /// List of artifact records
    pub module_records: Vec<maven::MavenArtifactRecord>,
}
register_class!("hudson.maven.reporters.MavenAggregatedArtifactRecord" => MavenAggregatedArtifactRecord);
impl Action for MavenAggregatedArtifactRecord {}

/// An action with a surefire test report
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurefireReport {
    /// Number of tests failed
    pub fail_count: u32,
    /// Number of tests skipped
    pub skip_count: u32,
    /// Number of tests
    pub total_count: u32,
    /// URL to the report
    pub url_name: String,
}
register_class!("hudson.maven.reporters.SurefireReport" => SurefireReport);
impl Action for SurefireReport {}

/// An action with a surefire test report aggregated from other reports
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurefireAggregatedReport {
    /// Number of tests failed
    pub fail_count: u32,
    /// Number of tests skipped
    pub skip_count: u32,
    /// Number of tests
    pub total_count: u32,
    /// URL to the report
    pub url_name: String,
}
register_class!("hudson.maven.reporters.SurefireAggregatedReport" => SurefireAggregatedReport);
impl Action for SurefireAggregatedReport {}

/// An action marking an approval on a pipeline
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PipelineApproverAction {
    /// User ID
    pub user_id: String,
}
register_class!("org.jenkinsci.plugins.workflow.support.steps.input.ApproverAction" => PipelineApproverAction);
impl Action for PipelineApproverAction {}
