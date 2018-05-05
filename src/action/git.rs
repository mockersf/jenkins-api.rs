//! Types related to git

use serde::Deserializer;

use std::collections::HashMap;

/// Describe a git branch
#[derive(Deserialize, Debug)]
pub struct Branch {
    /// SHA1 of the branch
    #[serde(rename = "SHA1")]
    sha1: String,
    /// Name of the branch
    name: String,
}

/// Revision from git
#[derive(Deserialize, Debug)]
pub struct Revision {
    /// SHA1 of the revision
    #[serde(rename = "SHA1")]
    sha1: String,
    /// Branch information
    branch: Vec<Branch>,
}

tagged_enum_or_default!(
    /// Information about a build related to a branch
    pub enum BranchBuild {
        /// Build from a git branch
        GitBuild (_class = "hudson.plugins.git.util.Build") {
            /// Revision
            revision: Revision,
            /// Build number
            build_number: u32,
            /// Build result
            build_result: Option<::build::BuildStatus>,
            /// Marked revision
            marked: Revision,
        },
    }
);

/// HashMap of builds by branch name
#[derive(Deserialize, Debug)]
pub struct BuildsByBranch {
    /// HashMap of builds by branch name
    #[serde(flatten)]
    branches: HashMap<String, BranchBuild>,
}
