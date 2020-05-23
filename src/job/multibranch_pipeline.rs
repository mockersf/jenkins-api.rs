use serde::Deserialize;

use crate::helpers::Class;

use super::Job;
use crate::action::CommonAction;
use crate::build::{CommonBuild, ShortBuild};
use crate::job::ShortJob;

job_base_with_common_fields_and_impl!(
    /// A pipeline project
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkflowMultiBranchProject {
        /// List of the jobs in the pipline
        pub jobs: Vec<ShortJob>,
    }
);
register_class!("org.jenkinsci.plugins.workflow.multibranch.WorkflowMultiBranchProject" => WorkflowMultiBranchProject);

impl WorkflowMultiBranchProject {}
