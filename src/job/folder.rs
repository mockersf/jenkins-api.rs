use serde::Deserialize;

use crate::helpers::Class;

use super::Job;
use crate::action::CommonAction;
use crate::build::{CommonBuild, ShortBuild};
use crate::job::ShortJob;

job_base_with_common_fields_and_impl!(
    /// A folder
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Folder {
        /// List of the jobs in the folder
        pub jobs: Vec<ShortJob>,
    }
);
register_class!("com.cloudbees.hudson.plugins.folder.Folder" => Folder);

impl Folder {}
