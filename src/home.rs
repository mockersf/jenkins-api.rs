//! Jenkins Home, describing state of the master

use serde::Deserialize;

use crate::client_internals::{Path, Result};
use crate::job::ShortJob;
use crate::view::ShortView;
use crate::Jenkins;

/// Describe how Jenkins allocates jobs to agents
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    /// Any job can be started on this node
    Normal,
    /// Only jobs specifically specifying this node can start
    Exclusive,
}

/// Index of Jenkins, with details about the master, a list of `Job` and a list of `View`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    /// Mode of the node for job selections
    pub mode: Mode,
    /// Description of the node
    pub node_description: String,
    /// Name of the node
    pub node_name: String,
    /// Number of executors of the node
    pub num_executors: u32,
    /// Description of the master
    pub description: Option<String>,
    /// List of jobs
    pub jobs: Vec<ShortJob>,
    /// Is Jenkins preparing to restart
    pub quieting_down: bool,
    /// HTTP port to the slave agent
    pub slave_agent_port: u32,
    /// Does this instance use crumbs for CSRF
    pub use_crumbs: bool,
    /// False if this instance is either UNSECURED or NO_AUTHENTICATION
    pub use_security: bool,
    /// List of views
    pub views: Vec<ShortView>,
}

impl Jenkins {
    /// Get Jenkins `Home`
    pub fn get_home(&self) -> Result<Home> {
        Ok(self.get(&Path::Home)?.json()?)
    }
}
