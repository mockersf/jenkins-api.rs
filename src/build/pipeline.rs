use helpers::Class;

use super::{Artifact, Build, BuildStatus, ShortBuild};
use action::CommonAction;
use changeset;
use job::WorkflowJob;

build_with_common_fields_and_impl!(
    /// A `Build` from a WorkflowJob
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkflowRun<ParentJob = WorkflowJob> {
        /// Change set for this build
        pub change_sets: Vec<changeset::CommonChangeSetList>,
        /// Previous build
        pub previous_build: Option<ShortBuild>,
    }
);
register_class!("org.jenkinsci.plugins.workflow.job.WorkflowRun" => WorkflowRun);

impl WorkflowRun {}
