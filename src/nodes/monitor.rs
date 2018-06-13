//! Types to parse the monitor data of a `Computer`

use serde;
use serde_json;

use helpers::Class;

/// Monitor data provided by Jenkins about a `Computer`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Data {
    /// A `String`, used for example by monitor `hudson.node_monitors.ArchitectureMonitor`
    String(String),
    /// A structured monitor
    MonitorData(CommonMonitorData),
    /// An empty monitor, meaning it was not able to retrieve data
    Empty,
}

/// Trait implemented by specialization of monitor data
pub trait MonitorData {}

/// A `MonitorData` on a `Computer`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonMonitorData {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonMonitorData => MonitorData);
impl MonitorData for CommonMonitorData {}

/// Swap Space Monitor
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapSpaceMonitor {
    /// Available physical memory
    pub available_physical_memory: u64,
    /// Available swap space
    pub available_swap_space: u64,
    /// Total physical memory
    pub total_physical_memory: u64,
    /// Total swap space
    pub total_swap_space: u64,
}
register_class!("hudson.node_monitors.SwapSpaceMonitor$MemoryUsage2" => SwapSpaceMonitor);
impl MonitorData for SwapSpaceMonitor {}

/// Swap Space Monitor
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpaceMonitorDescriptor {
    /// Timestamp
    pub timestamp: u64,
    /// Path monitored
    pub path: String,
    /// Size used
    pub size: u64,
}
register_class!("hudson.node_monitors.DiskSpaceMonitorDescriptor$DiskSpace" => DiskSpaceMonitorDescriptor);
impl MonitorData for DiskSpaceMonitorDescriptor {}

/// Response Time Monitor
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTimeMonitor {
    /// Timestamp
    pub timestamp: u64,
    /// Average response time
    pub average: u64,
}
register_class!("hudson.node_monitors.ResponseTimeMonitor$Data" => ResponseTimeMonitor);
impl MonitorData for ResponseTimeMonitor {}

/// Clock Difference Monitor
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClockDifference {
    /// Clock difference
    pub diff: i64,
}
register_class!("hudson.util.ClockDifference" => ClockDifference);
impl MonitorData for ClockDifference {}
