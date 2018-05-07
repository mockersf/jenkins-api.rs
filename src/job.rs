use failure::Error;

use build::ShortBuild;
use queue::ShortQueueItem;
use Jenkins;
use client::{self, Name, Path};
use job_builder::JobBuilder;

/// Ball Color corresponding to a `BuildStatus`
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BallColor {
    /// Success
    Blue,
    /// Success, and build is on-going
    BlueAnime,
    /// Unstable
    Yellow,
    /// Unstable, and build is on-going
    YellowAnime,
    /// Failure
    Red,
    /// Failure, and build is on-going
    RedAnime,
    /// Catch-all for disabled, aborted, not yet build
    Grey,
    /// Catch-all for disabled, aborted, not yet build, and build is on-going
    GreyAnime,
    /// Disabled
    Disabled,
    /// Disabled, and build is on-going
    DisabledAnime,
    /// Aborted
    Aborted,
    ///Aborted, and build is on-going
    AbortedAnime,
    /// Not Build
    #[serde(rename = "notbuilt")]
    NotBuilt,
    /// Not Build, and build is on-going
    #[serde(rename = "notbuilt_anime")]
    NotBuiltAnime,
}

/// Short Job that is used in lists and links from other structs
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortJob {
    /// Name of the job
    pub name: String,
    /// URL for the job
    pub url: String,
    /// Ball Color for the status of the job
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
    /// Name of the job
    pub name: String,
    /// Display Name of the job
    pub display_name: String,
    /// Full Display Name of the job
    pub full_display_name: String,
    /// Full Name of the job
    pub full_name: String,
    /// Description of the job
    pub description: String,
    /// URL for the job
    pub url: String,
    /// Ball Color for the status of the job
    pub color: BallColor,
    /// Is the job buildable?
    pub buildable: bool,
    /// Is concurrent build enabled for the job?
    pub concurrent_build: bool,
    /// Are dependencies kept for this job?
    pub keep_dependencies: bool,
    /// Next build number
    pub next_build_number: u32,
    /// Is this job currently in build queue
    pub in_queue: bool,
    /// Link to the last build
    pub last_build: Option<ShortBuild>,
    /// Link to the first build
    pub first_build: Option<ShortBuild>,
    /// Link to the last stable build
    pub last_stable_build: Option<ShortBuild>,
    /// Link to the last unstable build
    pub last_unstable_build: Option<ShortBuild>,
    /// Link to the last successful build
    pub last_successful_build: Option<ShortBuild>,
    /// Link to the last unsucressful build
    pub last_unsuccessful_build: Option<ShortBuild>,
    /// Link to the last complete build
    pub last_completed_build: Option<ShortBuild>,
    /// Link to the last failed build
    pub last_failed_build: Option<ShortBuild>,
    /// List of builds of the job
    pub builds: Vec<ShortBuild>,
}
impl Job {
    /// Enable a `Job`. It may need to be refreshed as it may have been updated
    pub fn enable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
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

    /// Disable a `Job`. It may need to be refreshed as it may have been updated
    pub fn disable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
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

    /// Build this job
    pub fn build(&self, jenkins_client: &Jenkins) -> Result<ShortQueueItem, Error> {
        self.builder(jenkins_client)?.send()
    }

    /// Create a `JobBuilder` to setup a build of a `Job`
    pub fn builder<'a, 'b, 'c, 'd>(
        &'a self,
        jenkins_client: &'b Jenkins,
    ) -> Result<JobBuilder<'a, 'b, 'c, 'd>, Error> {
        JobBuilder::new(self, jenkins_client)
    }

    /// Trigger a build remotely
    pub fn trigger_remotely(
        &self,
        jenkins_client: &Jenkins,
        token: &str,
        cause: Option<&str>,
    ) -> Result<ShortQueueItem, Error> {
        self.builder(jenkins_client)?
            .remotely_with_token_and_cause(token, cause)
            .send()
    }

    /// Poll configured SCM for changes
    pub fn poll_scm(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::PollSCMJob { name })?;
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
    /// Get a `Job` from it's `job_name`
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job {
            name: Name::Name(job_name),
        })?
            .json()?)
    }

    /// Build a `Job` from it's `job_name`
    pub fn build_job(&self, job_name: &str) -> Result<ShortQueueItem, Error> {
        JobBuilder::new_from_job_name(job_name, self)?.send()
    }

    /// Trigger a `Job` remotely from it's `job_name`
    pub fn trigger_job_remotely(
        &self,
        job_name: &str,
        token: &str,
        cause: Option<&str>,
    ) -> Result<ShortQueueItem, Error> {
        JobBuilder::new_from_job_name(job_name, self)?
            .remotely_with_token_and_cause(token, cause)
            .send()
    }

    /// Create a `JobBuilder` to setup a build of a `Job` from it's `job_name`
    pub fn job_builder<'a, 'b, 'c, 'd>(
        &'b self,
        job_name: &'a str,
    ) -> Result<JobBuilder<'a, 'b, 'c, 'd>, Error> {
        JobBuilder::new_from_job_name(job_name, self)
    }

    /// Poll SCM of a `Job` from it's `job_name`
    pub fn poll_scm_job(&self, job_name: &str) -> Result<(), Error> {
        self.post(&Path::PollSCMJob {
            name: Name::Name(job_name),
        })?;
        Ok(())
    }
}
