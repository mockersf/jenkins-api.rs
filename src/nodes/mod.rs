//! Jenkins Slaves Informations

use failure::Error;

use client_internals::{Name, Path};
use Jenkins;

pub mod computer;
pub mod monitor;

/// List of `Computer` associated to the `Jenkins` instance
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputerSet {
    /// Display name of the set
    pub display_name: String,
    /// Number of busy executors
    pub busy_executors: u32,
    /// Number of executors
    pub total_executors: u32,
    /// List of computers
    #[serde(rename = "computer")]
    pub computers: Vec<computer::CommonComputer>,
}

impl Jenkins {
    /// Get a `ComputerSet`
    pub fn get_nodes(&self) -> Result<ComputerSet, Error> {
        Ok(self.get(&Path::Computers)?.json()?)
    }

    /// Get a `Computer`
    pub fn get_node<'a, C>(&self, computer_name: C) -> Result<computer::CommonComputer, Error>
    where
        C: Into<computer::ComputerName<'a>>,
    {
        Ok(self.get(&Path::Computer {
            name: Name::Name(&computer_name.into().0),
        })?
            .json()?)
    }

    /// Get the master `Computer`
    pub fn get_master_node(&self) -> Result<computer::MasterComputer, Error> {
        Ok(self.get(&Path::Computer {
            name: Name::Name("(master)"),
        })?
            .json()?)
    }
}
