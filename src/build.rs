use failure::Error;

use job::Job;
use action::Action;
use Jenkins;
use client::{self, Name, Path};

/// Short Build that is used in lists and links from other structs
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortBuild {
    /// URL for the build
    pub url: String,
    /// Build number
    pub number: u32,
}
impl ShortBuild {
    /// Get the full details of a `Build` matching the `ShortBuild`
    pub fn get_full_build(&self, jenkins_client: &Jenkins) -> Result<Build, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Build { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::Build,
            }.into())
        }
    }
}

/// Status of a build
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildStatus {
    /// Successful build
    Success,
    /// Unstable build
    Unstable,
    /// Failed build
    Failure,
    /// Not yet built
    NotBuilt,
    /// Aborted build
    Aborted,
}

/// A `Build` of a `Job`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    /// URL for the build
    pub url: String,
    /// Build number for this job
    pub number: u32,
    /// Estimated duration
    pub estimated_duration: u32,
    /// Timestamp of the build start
    pub timestamp: u64,
    /// Are the logs kept?
    pub keep_log: bool,
    /// Build result
    pub result: BuildStatus,
    /// Display name, usually "#" followed by the build number
    pub display_name: String,
    /// Full display name: job name followed by the build display name
    pub full_display_name: String,
    /// Is this build currently running
    pub building: bool,
    /// Which slave was it build on
    pub built_on: String,
    /// Build number in string format
    pub id: String,
    /// ID while in the build queue
    pub queue_id: u32,
    /// Build actions
    pub actions: Vec<Action>,
    /// Change set for this build
    pub change_set: changeset::ChangeSetList,
}
impl Build {
    /// Get the `Job` from a `Build`
    pub fn get_job(&self, jenkins_client: &Jenkins) -> Result<Job, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Build { job_name, .. } = path {
            Ok(jenkins_client.get(&Path::Job { name: job_name })?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::Build,
            }.into())
        }
    }

    /// Get the console output from a `Build`
    pub fn get_console(&self, jenkins_client: &Jenkins) -> Result<String, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Build { job_name, number } = path {
            Ok(jenkins_client
                .get(&Path::ConsoleText { job_name, number })?
                .text()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::Build,
            }.into())
        }
    }
}

impl Jenkins {
    /// Get a build from a `job_name` and `build_number`
    pub fn get_build(&self, job_name: &str, build_number: u32) -> Result<Build, Error> {
        Ok(self.get(&Path::Build {
            job_name: Name::Name(job_name),
            number: build_number,
        })?
            .json()?)
    }
}

pub mod changeset {
    //! Types describing changes between two builds

    use serde::Deserializer;

    use user::ShortUser;

    tagged_enum_or_default!(
        /// List of changes found
        pub enum ChangeSetList {
            /// No changes recorded
            EmptyChangeSet (_class = "hudson.scm.EmptyChangeLogSet") {
            },
            /// Changes found from git
            GitChangeSetList (_class = "hudson.plugins.git.GitChangeSetList") {
                /// Origin of the changes
                kind: String,
                /// Changes in this list
                items: Vec<ChangeSet>,
            },
        }
    );

    tagged_enum_or_default!(
        /// Changes found
        pub enum ChangeSet {
            /// Changes found from git
            GitChangeSet (_class = "hudson.plugins.git.GitChangeSet") {
                /// Comment
                comment: String,
                /// Email of the commit
                author_email: String,
                /// ID of the commit
                commit_id: String,
                /// Date of the commit
                date: String,
                /// Commit message
                msg: String,
                /// Timestamp of the commit
                timestamp: u64,
                /// ID of the commit
                id: String,
                /// Files changed in the commit
                affected_paths: Vec<String>,
                /// Author of the commit
                author: ShortUser,
                /// Files changed in the commit, and how
                paths: Vec<PathChange>,
            },
        }
    );

    /// Edit type on a file
    #[derive(Debug, Deserialize, Clone, Copy)]
    #[serde(rename_all = "lowercase")]
    pub enum EditType {
        /// Adding a new file
        Add,
        /// Editing a file
        Edit,
        /// Deleting a file
        Delete,
    }

    /// A file that was changed
    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PathChange {
        /// File that was changed
        pub file: String,
        /// How it was changed
        pub edit_type: EditType,
    }

}
