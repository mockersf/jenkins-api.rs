//! Nodes found in a pipeline

use serde::{self, Deserialize, Serialize};

use crate::helpers::Class;

/// Trait implemented by specialization of PipelineNode
pub trait PipelineNode {}

/// A node of a pipeline
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonPipelineNode {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,

    #[cfg(not(feature = "extra-fields-visibility"))]
    #[serde(flatten)]
    extra_fields: serde_json::Value,
    #[cfg(feature = "extra-fields-visibility")]
    /// Extra fields not parsed for a common object
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}
specialize!(CommonPipelineNode => PipelineNode);
impl PipelineNode for CommonPipelineNode {}

/// Beginning of a flow
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlowStartNode {}
register_class!("org.jenkinsci.plugins.workflow.graph.FlowStartNode" => FlowStartNode);
impl PipelineNode for FlowStartNode {}

/// Beginning of a step
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepStartNode {}
register_class!("org.jenkinsci.plugins.workflow.cps.nodes.StepStartNode" => StepStartNode);
impl PipelineNode for StepStartNode {}

/// A step
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepAtomNode {}
register_class!("org.jenkinsci.plugins.workflow.cps.nodes.StepAtomNode" => StepAtomNode);
impl PipelineNode for StepAtomNode {}

/// End of a step
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepEndNode {}
register_class!("org.jenkinsci.plugins.workflow.cps.nodes.StepEndNode" => StepEndNode);
impl PipelineNode for StepEndNode {}

/// End of a flow
#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlowEndNode {}
register_class!("org.jenkinsci.plugins.workflow.graph.FlowEndNode" => FlowEndNode);
impl PipelineNode for FlowEndNode {}
