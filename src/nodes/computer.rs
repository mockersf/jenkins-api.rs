//! Types to parse a `Computer`

use std::collections::HashMap;

use serde;
use serde_json;

use helpers::Class;

use super::monitor;

/// Helper type to act on a `Computer`
#[derive(Debug)]
pub struct ComputerName<'a>(pub &'a str);
impl<'a> From<&'a str> for ComputerName<'a> {
    fn from(v: &'a str) -> ComputerName<'a> {
        ComputerName(v)
    }
}
impl<'a> From<&'a String> for ComputerName<'a> {
    fn from(v: &'a String) -> ComputerName<'a> {
        ComputerName(v)
    }
}

/// Trait implemented by specialization of computers
pub trait Computer {}

macro_rules! computer_with_common_fields_and_impl {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $field_type:ty,
            )*
            $(private_fields {
                $(
                    $(#[$private_field_attr:meta])*
                    $private_field:ident: $private_field_type:ty
                ),* $(,)*
            })*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            /// Name of the computer
            pub display_name: String,
            /// Description of the computer
            pub description: String,
            /// Icon for the computer
            pub icon: String,
            /// Icon for the computer
            pub icon_class_name: String,
            /// Is the computer idle
            pub idle: bool,
            /// Is the computer connected to master through JNLP
            pub jnlp_agent: bool,
            /// Can the computer launch a `Job`
            pub launch_supported: bool,
            /// Can a user launch a `Job` on this computer
            pub manual_launch_allowed: bool,
            /// Numbero of executors
            pub num_executors: u32,
            /// Is the computer offline
            pub offline: bool,
            /// Why is the computer offline
            pub offline_cause: Option<monitor::CommonMonitorData>,
            /// Why is the computer offline
            pub offline_cause_reason: Option<String>,
            /// Is the computer temporarily offline
            pub temporarily_offline: bool,
            /// Monitor data provided by the computer
            pub monitor_data: HashMap<String, monitor::Data>,
            /// Executors of the computer
            pub executors: Vec<Executor>,
            /// One off executors of the computer
            pub one_off_executors: Vec<Executor>,

            // TODO: actions, assignedLabels, loadStatistics 

            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
            $($(
                $(#[$private_field_attr])*
                $private_field: $private_field_type,
            )*)*
        }
        impl Computer for $name {}
    };
}

computer_with_common_fields_and_impl!(/// A Jenkins `Computer`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonComputer {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    private_fields {
        #[serde(flatten)]
        other_fields: serde_json::Value,
    }
});
specialize!(CommonComputer => Computer);

computer_with_common_fields_and_impl!(/// The master computer
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MasterComputer {});
register_class!("hudson.model.Hudson$MasterComputer" => MasterComputer);

computer_with_common_fields_and_impl!(/// A slave computer
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlaveComputer {});
register_class!("hudson.slave.SlaveComputer" => SlaveComputer);

/// An `Executor` of a `Computer`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Executor {
    /// An `Executor` of a `Computer`
    #[serde(rename_all = "camelCase")]
    Executor {
        /// `Build` that is currently running
        current_executable: Option<::build::ShortBuild>,
        /// Is it likely stuck
        likely_stuck: bool,
        /// Executor number
        number: u32,
        /// Progress in current executable
        progress: ExecutorProgress,
    },
    /// No data was retrieved about current executor, probably due to not
    /// enough depth in request
    MissingData {},
}

/// Progress in an executable
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ExecutorProgress {
    /// Percent done
    Percent(u32),
    /// Nothing
    None(i32),
}
