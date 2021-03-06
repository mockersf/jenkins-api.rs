use serde::Deserialize;

use crate::helpers::Class;

use super::{Artifact, Build, BuildStatus, ShortBuild};
use crate::action::CommonAction;
use crate::changeset;
use crate::job::{MatrixConfiguration, MatrixProject};
use crate::user::ShortUser;

build_with_common_fields_and_impl!(
    /// A `Build` from a MatrixProject
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MatrixBuild<ParentJob = MatrixProject> {
        /// Change set for this build
        pub change_set: changeset::CommonChangeSetList,
        /// Runs of each configuration
        pub runs: Vec<ShortBuild<MatrixRun>>,
        /// Which slave was it build on
        pub built_on: String,
        /// List of user ids who made a change since the last non-broken build
        pub culprits: Vec<ShortUser>,
    }
);
register_class!("hudson.matrix.MatrixBuild" => MatrixBuild);

impl MatrixBuild {}

build_with_common_fields_and_impl!(
    /// A `Build` from a MatrixConfiguration
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MatrixRun<ParentJob = MatrixConfiguration> {
        /// Change set for this build
        pub change_set: changeset::CommonChangeSetList,
        /// Which slave was it build on
        pub built_on: String,
        /// List of user ids who made a change since the last non-broken build
        pub culprits: Vec<ShortUser>,
    }
);
register_class!("hudson.matrix.MatrixRun" => MatrixRun);

impl MatrixRun {}
