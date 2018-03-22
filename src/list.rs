use failure::Error;

use jobs::Job;
use super::Jenkins;
use super::client::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    Normal,
    Exclusive,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    mode: Mode,
    node_description: String,
    node_name: String,
    num_executors: i32,
    description: Option<String>,
    jobs: Vec<Job>,
    quieting_down: bool,
    slave_agent_port: i32,
    use_crumbs: bool,
    use_security: bool,
    views: Vec<View>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListView {
    description: String,
    name: String,
    url: String,
    jobs: Vec<Job>,
}

impl Jenkins {
    pub fn get_home(&self) -> Result<Home, Error> {
        Ok(self.get(&Path::Home).send()?.error_for_status()?.json()?)
    }

    pub fn get_view(&self, view_name: &str) -> Result<View, Error> {
        Ok(self.get(&Path::View { name: view_name })
            .send()?
            .error_for_status()?
            .json()?)
    }
}
