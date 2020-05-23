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

/// Port announced by Jenkins for agent to connect to
#[derive(Debug, Copy, Clone)]
pub enum AgentPort {
    /// TCP connection from agent is disabled
    Disabled,
    /// Port is selected randomly
    Random,
    /// Port is fixed
    Fixed(u32),
}

impl<'de> Deserialize<'de> for AgentPort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<AgentPort, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_i32(AgentPortI32Visitor)
    }
}
struct AgentPortI32Visitor;
impl<'de> serde::de::Visitor<'de> for AgentPortI32Visitor {
    type Value = AgentPort;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -1 and 65536")
    }

    fn visit_i8<E>(self, value: i8) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_i16<E>(self, value: i16) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_i32<E>(self, value: i32) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_i64<E>(self, value: i64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            -1 => Ok(AgentPort::Disabled),
            0 => Ok(AgentPort::Random),
            p if p > 0 && p < 65537 => Ok(AgentPort::Fixed(p as u32)),
            x => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(x),
                &"port between -1 and 65536",
            )),
        }
    }

    fn visit_u8<E>(self, value: u8) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_u16<E>(self, value: u16) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_u32<E>(self, value: u32) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(i64::from(value))
    }

    fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(AgentPort::Random),
            p if p > 0 && p < 65537 => Ok(AgentPort::Fixed(p as u32)),
            x => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(x),
                &"port between -1 and 65536",
            )),
        }
    }
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
    pub slave_agent_port: AgentPort,
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
