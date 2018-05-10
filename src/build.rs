use failure::Error;
use serde::Deserializer;

use Jenkins;
use action::Action;
use client::{self, Name, Path};
use job::Job;
use user::ShortUser;

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

tagged_enum_or_default!(

    /// A `Build` of a `Job`
    pub enum Build {
        /// A `Build` from a FreeStyleProject
        FreeStyleBuild (_class = "hudson.model.FreeStyleBuild") {
            /// URL for the build
            url: String,
            /// Build number for this job
            number: u32,
            /// Duration
            duration: u32,
            /// Estimated duration
            estimated_duration: u32,
            /// Timestamp of the build start
            timestamp: u64,
            /// Are the logs kept?
            keep_log: bool,
            /// Build result
            result: BuildStatus,
            /// Display name, usually "#" followed by the build number
            display_name: String,
            /// Full display name: job name followed by the build display name
            full_display_name: String,
            /// Is this build currently running
            building: bool,
            /// Which slave was it build on
            built_on: String,
            /// Build number in string format
            id: String,
            /// ID while in the build queue
            queue_id: u32,
            /// Build actions
            actions: Vec<Action>,
            /// Change set for this build
            change_set: changeset::ChangeSetList,
        },
        /// A `Build` from a WorkflowJob
        WorkflowRun (_class = "org.jenkinsci.plugins.workflow.job.WorkflowRun") {
            /// URL for the build
            url: String,
            /// Build number for this job
            number: u32,
            /// Duration
            duration: u32,
            /// Estimated duration
            estimated_duration: u32,
            /// Timestamp of the build start
            timestamp: u64,
            /// Are the logs kept?
            keep_log: bool,
            /// Build result
            result: BuildStatus,
            /// Display name, usually "#" followed by the build number
            display_name: String,
            /// Full display name: job name followed by the build display name
            full_display_name: String,
            /// Is this build currently running
            building: bool,
            /// Build number in string format
            id: String,
            /// ID while in the build queue
            queue_id: u32,
            /// Build actions
            actions: Vec<Action>,
            /// Change set for this build
            change_sets: Vec<changeset::ChangeSetList>,
            /// Culprits
            culprits: Vec<ShortUser>,
            /// Previous build
            previous_build: Option<ShortBuild>,
        },
    }
);

macro_rules! build_common_fields_dispatch {
    ($field:ident -> $return:ty) => {
        pub(crate) fn $field(&self) -> Result<$return, Error> {
            match self {
                &Build::FreeStyleBuild { ref $field, .. } => Ok($field),
                &Build::WorkflowRun { ref $field, .. } => Ok($field),
                x @ &Build::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::Build,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &Build::FreeStyleBuild { $field, .. } => Ok($field),
                &Build::WorkflowRun { $field, .. } => Ok($field),
                x @ &Build::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::Build,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub ref $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &Build::FreeStyleBuild { ref $field, .. } => Ok($field),
                &Build::WorkflowRun { ref $field, .. } => Ok($field),
                x @ &Build::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::Build,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
}

impl Build {
    build_common_fields_dispatch!(url -> &str);
    build_common_fields_dispatch!(
        /// Get timestamp of a build
        pub timestamp -> u64
    );
    build_common_fields_dispatch!(
        /// Get result of a build
        pub result -> BuildStatus
    );
    build_common_fields_dispatch!(
        /// Get number of a build
        pub number -> u32
    );
    build_common_fields_dispatch!(
        /// Get duration of a build
        pub duration -> u32
    );

    /// Get the `Job` from a `Build`
    pub fn get_job(&self, jenkins_client: &Jenkins) -> Result<Job, Error> {
        let path = jenkins_client.url_to_path(&self.url()?);
        if let Path::Build { job_name, .. } = path {
            Ok(jenkins_client.get(&Path::Job { name: job_name })?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url()?.to_string(),
                expected: client::error::ExpectedType::Build,
            }.into())
        }
    }

    /// Get the console output from a `Build`
    pub fn get_console(&self, jenkins_client: &Jenkins) -> Result<String, Error> {
        let path = jenkins_client.url_to_path(&self.url()?);
        if let Path::Build { job_name, number } = path {
            Ok(jenkins_client
                .get(&Path::ConsoleText { job_name, number })?
                .text()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url()?.to_string(),
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
            /// Changes found from a repo
            RepoChangeLogSet (_class = "hudson.plugins.repo.RepoChangeLogSet") {
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
            /// Changes found from a repo
            ChangeLogEntry (_class = "hudson.plugins.repo.ChangeLogEntry") {
                /// ID of the commit
                commit_id: Option<String>,
                /// Commit message
                msg: String,
                /// Timestamp of the commit
                timestamp: i64,
                /// Files changed in the commit
                affected_paths: Vec<String>,
                /// Author of the commit
                author: ShortUser,
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
