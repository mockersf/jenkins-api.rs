use serde::Deserialize;

use crate::helpers::Class;

use super::{Job, ShortJob};
use crate::action::CommonAction;

/// A pipeline project
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowMultiBranchProject {
    /// Name of the job
    pub name: String,
    /// Display Name of the job
    pub display_name: String,
    /// Full Display Name of the job
    pub full_display_name: Option<String>,
    /// Full Name of the job
    pub full_name: Option<String>,
    /// Display Name of the job
    pub display_name_or_null: Option<String>,
    /// URL for the job
    pub url: String,
    /// Description of the job
    pub description: String,
    /// Actions of a job
    pub actions: Vec<Option<CommonAction>>,
    /// List of the jobs
    pub jobs: Vec<ShortJob>,
}

register_class!("org.jenkinsci.plugins.workflow.multibranch.WorkflowMultiBranchProject" => WorkflowMultiBranchProject);

impl Job for WorkflowMultiBranchProject {
    fn url(&self) -> &str {
        &self.url
    }

    fn name(&self) -> &str {
        &self.name
    }
}
