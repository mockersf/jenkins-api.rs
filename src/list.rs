use failure::Error;

use jobs::ShortJob;
use Jenkins;
use client::{Name, Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortView {
    pub name: String,
    pub url: String,
}
impl ShortView {
    pub fn get_full_view(&self, jenkins_client: &Jenkins) -> Result<View, Error> {
        jenkins_client.get_from_url(self.url.clone())
    }
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
    pub mode: Mode,
    pub node_description: String,
    pub node_name: String,
    pub num_executors: i32,
    pub description: Option<String>,
    pub jobs: Vec<ShortJob>,
    pub quieting_down: bool,
    pub slave_agent_port: i32,
    pub use_crumbs: bool,
    pub use_security: bool,
    pub views: Vec<ShortView>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub description: Option<String>,
    pub name: String,
    pub url: String,
    pub jobs: Vec<ShortJob>,
}

impl Jenkins {
    pub fn get_home(&self) -> Result<Home, Error> {
        Ok(self.get(&Path::Home).send()?.error_for_status()?.json()?)
    }

    pub fn get_view(&self, view_name: &str) -> Result<View, Error> {
        Ok(self.get(&Path::View {
            name: Name::Name(view_name),
        }).send()?
            .error_for_status()?
            .json()?)
    }
}
