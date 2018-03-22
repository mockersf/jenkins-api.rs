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
pub struct Build {
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
    last_build: Option<Build>,
    first_build: Option<Build>,
    last_stable_build: Option<Build>,
    last_unstable_build: Option<Build>,
    last_successful_build: Option<Build>,
    last_unsuccessful_build: Option<Build>,
    last_completed_build: Option<Build>,
    last_failed_build: Option<Build>,
    builds: Vec<Build>,
}

impl Jenkins {
    pub fn get_job(&self, job_name: &str) -> Result<Job, Error> {
        Ok(self.get(&Path::Job { name: job_name })
            .send()?
            .error_for_status()?
            .json()?)
    }
}
