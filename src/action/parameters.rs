//! Types to parse the parameters of a `Build`

use serde;
use serde_json;

use helpers::Class;

/// Trait implemented by specialization of Parameter
pub trait Parameter {}

/// A node of a pipeline
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonParameter {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    /// The parameter name
    pub name: String,

    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonParameter => Parameter);
impl Parameter for CommonParameter {}

/// A boolean parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BooleanParameterValue {
    /// The parameter name
    pub name: String,
    /// The parameter value
    pub value: bool,
}
register_class!("hudson.model.BooleanParameterValue" => BooleanParameterValue);
impl Parameter for BooleanParameterValue {}

/// A file parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileParameterValue {
    /// The parameter name
    pub name: String,
}
register_class!("hudson.model.FileParameterValue" => FileParameterValue);
impl Parameter for FileParameterValue {}

/// A password parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasswordParameterValue {
    /// The parameter name
    pub name: String,
}
register_class!("hudson.model.PasswordParameterValue" => PasswordParameterValue);
impl Parameter for PasswordParameterValue {}

/// A run parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunParameterValue {
    /// The parameter name
    pub name: String,
    /// Name of the `Job` for this parameter
    pub job_name: String,
    /// Number of the `Build` passed
    pub number: String,
}
register_class!("hudson.model.RunParameterValue" => RunParameterValue);
impl Parameter for RunParameterValue {}

/// A string parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StringParameterValue {
    /// The parameter name
    pub name: String,
    /// The parameter value
    pub value: String,
}
register_class!("hudson.model.StringParameterValue" => StringParameterValue);
impl Parameter for StringParameterValue {}

/// A text parameter
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextParameterValue {
    /// The parameter name
    pub name: String,
    /// The parameter value
    pub value: String,
}
register_class!("hudson.model.TextParameterValue" => TextParameterValue);
impl Parameter for TextParameterValue {}
