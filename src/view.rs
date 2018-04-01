use failure::Error;

use job::ShortJob;
use Jenkins;
use client::{self, Name, Path};

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

/// Short View that is used in lists and links from other structs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortView {
    /// Name of the view
    pub name: String,
    /// URL for the view
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
                expected: "View".to_string(),
            }.into())
        }
    }
}

/// A Jenkins `View` with a list of `ShortJob`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    /// Description of the view
    pub description: Option<String>,
    /// Name of the view
    pub name: String,
    /// URL for the view
    pub url: String,
    /// List of jobs in the view
    pub jobs: Vec<ShortJob>,
}
impl View {
    /// Add the job `job_name` to this view
    pub fn add_job(&self, jenkins_client: &Jenkins, job_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::AddJobToView {
                job_name: Name::Name(job_name),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "View".to_string(),
            }.into())
        }
    }

    /// Remove the job `job_name` from this view
    pub fn remove_job(&self, jenkins_client: &Jenkins, job_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::RemoveJobFromView {
                job_name: Name::Name(job_name),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: "View".to_string(),
            }.into())
        }
    }
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

    /// Add the job `job_name` to the view `view_name`
    pub fn add_job(&self, view_name: &str, job_name: &str) -> Result<(), Error> {
        self.post(&Path::AddJobToView {
            job_name: Name::Name(job_name),
            view_name: Name::Name(view_name),
        })?;
        Ok(())
    }

    /// Remove the job `job_name` from the view `view_name`
    pub fn remove_job(&self, view_name: &str, job_name: &str) -> Result<(), Error> {
        self.post(&Path::AddJobToView {
            job_name: Name::Name(job_name),
            view_name: Name::Name(view_name),
        })?;
        Ok(())
    }
}
