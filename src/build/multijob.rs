use helpers::Class;

use super::{Artifact, Build, BuildStatus};
use action::CommonAction;
/* use build::ShortBuild; */
use changeset;
use user::ShortUser;

build_with_common_fields_and_impl!(/// A `Build` from a MultiJobProject
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultiJobBuild {
    /// Change set for this build
    pub change_set: changeset::CommonChangeSetList,
    /// Which slave was it build on
    pub built_on: String,
    /// List of user ids who made a change since the last non-broken build
    pub culprits: Vec<ShortUser>,
    /// Sub-builds of multi job
    pub sub_builds: Vec<MultiJobSubBuild>,
});
register_class!("com.tikal.jenkins.plugins.multijob.MultiJobBuild" => MultiJobBuild);

impl MultiJobBuild {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A sub-build from a multi job project
pub struct MultiJobSubBuild {
    /// If build was aborted
    pub abort: bool,
    /// Build number
    pub build_number: u32,
    /// Duration of build
    pub duration: String,
    /// Icon of build
    pub icon: String,
    /// Name of job
    pub job_name: String,
    /// Build number for parent job
    pub parent_build_number: u32,
    /// Job name of parent
    pub parent_job_name: String,
    /// Name of phase
    pub phase_name: String,
    /// Resulting status of build
    pub result: Option<BuildStatus>,
    /// If build was retried
    pub retry: bool,
    /// Url of build
    pub url: String,
}
