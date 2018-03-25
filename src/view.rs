use failure::Error;

use job::ShortJob;
use Jenkins;
use client::{self, Name, Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortView {
    pub name: String,
    pub url: String,
}
impl ShortView {
    pub fn get_full_view(&self, jenkins_client: &Jenkins) -> Result<View, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "view".to_string(),
            }.into())
        }
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
        Ok(self.get(&Path::Home)?.json()?)
    }

    pub fn get_view(&self, view_name: &str) -> Result<View, Error> {
        Ok(self.get(&Path::View {
            name: Name::Name(view_name),
        })?
            .json()?)
    }
}
