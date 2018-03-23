use failure::Error;

use super::Jenkins;
use super::client::Path;

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
    name: String,
    url: String,
    color: BallColor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortBuild {
    url: String,
    number: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    name: String,
    display_name: String,
    full_display_name: String,
    full_name: String,
    description: String,
    url: String,
    color: BallColor,
    buildable: bool,
    concurrent_build: bool,
    keep_dependencies: bool,
    next_build_number: i32,
    in_queue: bool,
    last_build: Option<ShortBuild>,
    first_build: Option<ShortBuild>,
    last_stable_build: Option<ShortBuild>,
    last_unstable_build: Option<ShortBuild>,
    last_successful_build: Option<ShortBuild>,
    last_unsuccessful_build: Option<ShortBuild>,
    last_completed_build: Option<ShortBuild>,
    last_failed_build: Option<ShortBuild>,
    builds: Vec<ShortBuild>,
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
    url: String,
    number: i32,
    estimated_duration: u32,
    timestamp: u64,
    keep_log: bool,
    result: BuildStatus,
    display_name: String,
    full_display_name: String,
    building: bool,
    built_on: String,
    id: String,
    queue_id: u32,
}

impl Jenkins {
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job { name: job_name })
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn get_build(&self, job_name: &str, build_id: u32) -> Result<Build, Error> {
        Ok(self.get(&Path::Build {
            job_name: job_name,
            id: build_id,
        }).send()?
            .error_for_status()?
            .json()?)
    }
}
