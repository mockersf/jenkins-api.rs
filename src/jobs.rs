use failure::Error;

use super::Jenkins;
use super::client::{Name, Path};

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
        jenkins_client.get_from_url(self.url.clone())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortBuild {
    pub url: String,
    pub number: i32,
}
impl ShortBuild {
    pub fn get_full_build(&self, jenkins_client: &Jenkins) -> Result<Build, Error> {
        jenkins_client.get_from_url(self.url.clone())
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

impl Jenkins {
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job {
            name: Name::Name(job_name),
        }).send()?
            .error_for_status()?
            .json()?)
    }

    pub fn get_build(&self, job_name: &str, build_id: u32) -> Result<Build, Error> {
        Ok(self.get(&Path::Build {
            job_name: Name::Name(job_name),
            id: build_id,
        }).send()?
            .error_for_status()?
            .json()?)
    }
}
