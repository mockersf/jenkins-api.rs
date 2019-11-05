//! Types related to maven

use serde::Deserialize;

use crate::client;
use crate::client_internals::path::Path;
use crate::Jenkins;

/// Artifact produced by a build
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    /// Artifact ID
    pub artifact_id: String,
    /// Canonical name
    pub canonical_name: String,
    /// Classifier (sources, javadoc, ...)
    pub classifier: Option<String>,
    /// File name
    pub file_name: String,
    /// Group ID
    pub group_id: String,
    /// MD5 checksum
    pub md5sum: String,
    /// Artifact type (jar, war, javadoc, java-source, ...)
    #[serde(rename = "type")]
    pub artifact_type: String,
    /// Version
    pub version: String,
}

/// Short Maven Artifact Record that is returned when getting a maven build
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortMavenArtifactRecord {
    /// URL to the full report
    pub url: String,
}
impl ShortMavenArtifactRecord {
    /// Get the full report of a `MavenArtifactRecord` matching the `ShortMavenArtifactRecord`
    pub fn get_full_artifact_record(
        &self,
        jenkins_client: &Jenkins,
    ) -> Result<MavenArtifactRecord, Box<dyn std::error::Error>> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::MavenArtifactRecord { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::MavenArtifactRecord,
            }
            .into())
        }
    }
}

/// Describe the artifacts produced by a build
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactRecord {
    /// URL to this record
    pub url: String,
    /// List of the artifacts
    pub attached_artifacts: Vec<Artifact>,
    /// Main artifact
    pub main_artifact: Artifact,
    /// Parent build
    pub parent: crate::build::ShortBuild,
    /// POM artifact
    pub pom_artifact: Artifact,
}
