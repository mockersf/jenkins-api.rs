use failure::Error;

use build::ShortBuild;
use super::Jenkins;
use super::client::{Name, Path};
use super::error;

#[derive(Debug, Deserialize)]
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
    #[serde(rename = "notbuilt")] NotBuilt,
    #[serde(rename = "notbuilt_anime")] NotBuiltAnime,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortJob {
    pub name: String,
    pub url: String,
    pub color: BallColor,
}
impl ShortJob {
    pub fn get_full_job(&self, jenkins_client: &Jenkins) -> Result<Job, Error> {
        jenkins_client.get_from_url(&self.url)
    }
}

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
    pub next_build_number: i32,
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
    pub fn enable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::JobEnable { name })?;
            Ok(())
        } else {
            Err(error::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "job".to_string(),
            }.into())
        }
    }

    pub fn disable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { name } = path {
            jenkins_client.post(&Path::JobDisable { name })?;
            Ok(())
        } else {
            Err(error::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "job".to_string(),
            }.into())
        }
    }
}

impl Jenkins {
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job {
            name: Name::Name(job_name),
        }).send()?
            .error_for_status()?
            .json()?)
    }
}
