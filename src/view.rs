use failure::Error;

use job::ShortJob;
use Jenkins;
use client::{self, Name, Path};

/// Describe how Jenkins allocates jobs to agents
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    Normal,
    Exclusive,
}

/// Index of Jenkins, with details about the master, a list of `Job` and a list of `View`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    pub mode: Mode,
    pub node_description: String,
    pub node_name: String,
    pub num_executors: u32,
    pub description: Option<String>,
    pub jobs: Vec<ShortJob>,
    pub quieting_down: bool,
    pub slave_agent_port: u32,
    pub use_crumbs: bool,
    pub use_security: bool,
    pub views: Vec<ShortView>,
}

/// Short View that is used in lists and links from other structs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortView {
    pub name: String,
    pub url: String,
}
impl ShortView {
    /// Get the full details of a `View` matching the `ShortView`
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

/// A Jenkins `View` with a list of `ShortJob`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub description: Option<String>,
    pub name: String,
    pub url: String,
    pub jobs: Vec<ShortJob>,
}

impl Jenkins {
    /// Get Jenkins `Home`
    pub fn get_home(&self) -> Result<Home, Error> {
        Ok(self.get(&Path::Home)?.json()?)
    }

    /// Get a `View`
    pub fn get_view(&self, view_name: &str) -> Result<View, Error> {
        Ok(self.get(&Path::View {
            name: Name::Name(view_name),
        })?
            .json()?)
    }
}
