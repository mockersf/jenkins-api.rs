use helpers::Class;

use super::{Artifact, Build, BuildStatus};
use action::CommonAction;
use changeset;
use job::BuildFlowJob;
use user::ShortUser;

build_with_common_fields_and_impl!(
    /// A `Build` from a BuildFlowJob
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct BuildFlowRun<ParentJob = BuildFlowJob> {
        /// Change set for this build
        pub change_set: changeset::CommonChangeSetList,
        /// Which slave was it build on
        pub built_on: String,
        /// List of user ids who made a change since the last non-broken build
        pub culprits: Vec<ShortUser>,
    }
);
register_class!("com.cloudbees.plugins.flow.FlowRun" => BuildFlowRun);

impl BuildFlowRun {}
