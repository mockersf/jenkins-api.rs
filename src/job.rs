use failure::Error;

use build::ShortBuild;
use Jenkins;
use client::{self, Name, Path};

/// Ball Color corresponding to a `BuildStatus`
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BallColor {
    Blue,
    BlueAnime,
    Yellow,
    YellowAnime,
    Red,
    RedAnime,
    Grey,
    GreyAnime,
    Disabled,
    DisabledAnime,
    Aborted,
    AbortedAnime,
    #[serde(rename = "notbuilt")]
    NotBuilt,
    #[serde(rename = "notbuilt_anime")]
    NotBuiltAnime,
}

/// Short Job that is used in lists and links from other structs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortJob {
    pub name: String,
    pub url: String,
    pub color: BallColor,
}
impl ShortJob {
    /// Get the full details of a `Job` matching the `ShortJob`
    pub fn get_full_job(&self, jenkins_client: &Jenkins) -> Result<Job, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }
}

/// A Jenkins `Job`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub name: String,
    pub display_name: String,
    pub full_display_name: String,
    pub full_name: String,
    pub description: String,
    pub url: String,
    pub color: BallColor,
    pub buildable: bool,
    pub concurrent_build: bool,
    pub keep_dependencies: bool,
    pub next_build_number: u32,
    pub in_queue: bool,
    pub last_build: Option<ShortBuild>,
    pub first_build: Option<ShortBuild>,
    pub last_stable_build: Option<ShortBuild>,
    pub last_unstable_build: Option<ShortBuild>,
    pub last_successful_build: Option<ShortBuild>,
    pub last_unsuccessful_build: Option<ShortBuild>,
    pub last_completed_build: Option<ShortBuild>,
    pub last_failed_build: Option<ShortBuild>,
    pub builds: Vec<ShortBuild>,
}
impl Job {
    /// Enable a `Job`. This will consume the `Job` and it will need to be refreshed as it may have been updated
    pub fn enable(self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::JobEnable { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }

    /// Disable a `Job`. This will consume the `Job` and it will need to be refreshed as it may have been updated
    pub fn disable(self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::JobDisable { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }

    /// Add this job to the view `view_name`
    pub fn add_to_view(&self, jenkins_client: &Jenkins, view_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::AddJobToView {
                job_name: name,
                view_name: Name::Name(view_name),
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }

    /// Remove this job from the view `view_name`
    pub fn remove_from_view(&self, jenkins_client: &Jenkins, view_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::RemoveJobFromView {
                job_name: name,
                view_name: Name::Name(view_name),
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }
}

impl Jenkins {
    /// Get a job from it's `job_name`
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job {
            name: Name::Name(job_name),
        })?
            .json()?)
    }
}
