use failure::Error;

use job::ShortJob;
use Jenkins;
use client::{self, Path};

/// A queued item in Jenkins, with information about the `Job` and why / since when it's waiting
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub blocked: bool,
    pub buildable: bool,
    pub id: u32,
    pub in_queue_since: u64,
    pub params: String,
    pub stuck: bool,
    pub task: ShortJob,
    pub url: String,
    pub why: String,
    pub buildable_start_milliseconds: u64,
}
impl QueueItem {
    /// Refresh a `QueueItem`, consuming the existing one and returning a new `QueueItem`
    pub fn refresh_item(self, jenkins_client: &Jenkins) -> Result<Self, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::QueueItem { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "QueueItem".to_string(),
            }.into())
        }
    }
}

/// The Jenkins `Queue`, the list of `QueueItem` that are waiting to be built
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    pub items: Vec<QueueItem>,
}

impl Jenkins {
    /// Get the Jenkins items queue
    pub fn get_queue(&self) -> Result<Queue, Error> {
        Ok(self.get(&Path::Queue)?.json()?)
    }
}
