use failure::Error;

use jobs::Job;
use super::Jenkins;
use super::client::{Name, Path};
use super::error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortBuild {
    pub url: String,
    pub number: i32,
}
impl ShortBuild {
    pub fn get_full_build(&self, jenkins_client: &Jenkins) -> Result<Build, Error> {
        jenkins_client.get_from_url(&self.url)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildStatus {
    Success,
    Unstable,
    Failure,
    NotBuilt,
    Aborted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub url: String,
    pub number: i32,
    pub estimated_duration: u32,
    pub timestamp: u64,
    pub keep_log: bool,
    pub result: BuildStatus,
    pub display_name: String,
    pub full_display_name: String,
    pub building: bool,
    pub built_on: String,
    pub id: String,
    pub queue_id: u32,
}
impl Build {
    pub fn get_job(&self, jenkins_client: &Jenkins) -> Result<Job, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Build { job_name, .. } = path {
            Ok(jenkins_client
                .get(&Path::Job { name: job_name })
                .send()?
                .error_for_status()?
                .json()?)
        } else {
            Err(error::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "job".to_string(),
            }.into())
        }
    }
}

impl Jenkins {
    pub fn get_build(&self, job_name: &str, build_id: u32) -> Result<Build, Error> {
        Ok(self.get(&Path::Build {
            job_name: Name::Name(job_name),
            id: build_id,
        }).send()?
            .error_for_status()?
            .json()?)
    }
}
