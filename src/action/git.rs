//! Types related to git

use std::collections::HashMap;

use serde;
use serde_json;

use helpers::Class;

/// Describe a git branch
#[derive(Deserialize, Debug)]
pub struct Branch {
    /// SHA1 of the branch
    #[serde(rename = "SHA1")]
    pub sha1: String,
    /// Name of the branch
    pub name: String,
}

/// Revision from git
#[derive(Deserialize, Debug)]
pub struct Revision {
    /// SHA1 of the revision
    #[serde(rename = "SHA1")]
    pub sha1: String,
    /// Branch information
    pub branch: Vec<Branch>,
}

/// Trait implemented by specialization of BranchBuild
pub trait BranchBuild {}

/// Information about a build related to a branch
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonBranchBuild {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonBranchBuild => BranchBuild);
impl BranchBuild for CommonBranchBuild {}

/// Build from a git branch
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitBranchBuild {
    /// Revision
    pub revision: Revision,
    /// Build number
    pub build_number: u32,
    /// Build result
    pub build_result: Option<::build::BuildStatus>,
    /// Marked revision
    pub marked: Revision,
}
register_class!("hudson.plugins.git.util.Build" => GitBranchBuild);
impl BranchBuild for GitBranchBuild {}

/// HashMap of builds by branch name
#[derive(Deserialize, Debug)]
pub struct BuildsByBranch {
    /// HashMap of builds by branch name
    #[serde(flatten)]
    pub branches: HashMap<String, CommonBranchBuild>,
}
