use failure::Error;

use job::Job;
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
                expected: "Build".to_string(),
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
                expected: "Build".to_string(),
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
