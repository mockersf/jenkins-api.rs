//! Source Control Management configuration

mod browser;
pub use self::browser::*;
mod scm;
pub use self::scm::*;

/// SCM merge options
#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MergeOptions {
    /// Merge strategy
    merge_strategy: String,
    /// Fast forward mode
    fast_forward_mode: String,
    /// Merge target
    merge_target: Option<String>,
    /// Remote branch
    remote_branch_name: Option<String>,
}
