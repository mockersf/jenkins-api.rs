//! Jenkins Views, use to group Jobs

use failure::Error;
use serde;
use serde_json;

use helpers::Class;

use client::{self, Name, Path};
use job::{JobName, ShortJob};
use property::CommonProperty;
use Jenkins;

/// Short View that is used in lists and links from other structs
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortView {
    /// Name of the view
    pub name: String,
    /// URL for the view
    pub url: String,
    #[serde(flatten)]
    other_fields: Option<serde_json::Value>,
}

impl ShortView {
    /// Get the full details of a `View` matching the `ShortView`
    pub fn get_full_view(&self, jenkins_client: &Jenkins) -> Result<CommonView, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::View,
            }.into())
        }
    }
}

/// Helper type to act on a view
#[derive(Debug)]
pub struct ViewName<'a>(pub &'a str);
impl<'a> From<&'a str> for ViewName<'a> {
    fn from(v: &'a str) -> ViewName<'a> {
        ViewName(v)
    }
}
impl<'a> From<&'a String> for ViewName<'a> {
    fn from(v: &'a String) -> ViewName<'a> {
        ViewName(v)
    }
}
impl<'a> From<&'a ShortView> for ViewName<'a> {
    fn from(v: &'a ShortView) -> ViewName<'a> {
        ViewName(&v.name)
    }
}
impl<'a, T: View> From<&'a T> for ViewName<'a> {
    fn from(v: &'a T) -> ViewName<'a> {
        ViewName(v.name())
    }
}

/// Trait implemented by specialization of view
pub trait View {
    /// Get the name of the view
    fn name(&self) -> &str;
}

/// A Jenkins `View` with a list of `ShortJob`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonView {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    /// Description of the view
    pub description: Option<String>,
    /// Name of the view
    pub name: String,
    /// URL for the view
    pub url: String,
    /// List of jobs in the view
    pub jobs: Vec<ShortJob>,
    /// Properties of the view
    pub property: Vec<CommonProperty>,
    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonView => View);
impl View for CommonView {
    fn name(&self) -> &str {
        &self.name
    }
}

/// A Jenkins `View` with a list of `ShortJob`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListView {
    /// Description of the view
    pub description: Option<String>,
    /// Name of the view
    pub name: String,
    /// URL for the view
    pub url: String,
    /// List of jobs in the view
    pub jobs: Vec<ShortJob>,
    /// Properties of the view
    pub property: Vec<CommonProperty>,
}
register_class!("hudson.model.ListView" => ListView);
impl View for ListView {
    fn name(&self) -> &str {
        &self.name
    }
}

impl ListView {
    /// Add the job `job_name` to this view
    pub fn add_job<'a, J>(&self, jenkins_client: &Jenkins, job_name: J) -> Result<(), Error>
    where
        J: Into<JobName<'a>>,
    {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::AddJobToView {
                job_name: Name::Name(&job_name.into().0),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::View,
            }.into())
        }
    }

    /// Remove the job `job_name` from this view
    pub fn remove_job<'a, J>(&self, jenkins_client: &Jenkins, job_name: J) -> Result<(), Error>
    where
        J: Into<JobName<'a>>,
    {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::RemoveJobFromView {
                job_name: Name::Name(&job_name.into().0),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::View,
            }.into())
        }
    }
}

impl Jenkins {
    /// Get a `View`
    pub fn get_view<'a, V>(&self, view_name: V) -> Result<CommonView, Error>
    where
        V: Into<ViewName<'a>>,
    {
        Ok(self.get(&Path::View {
            name: Name::Name(&view_name.into().0),
        })?
            .json()?)
    }

    /// Add the job `job_name` to the view `view_name`
    pub fn add_job_to_view<'a, 'b, V, J>(&self, view_name: V, job_name: J) -> Result<(), Error>
    where
        V: Into<ViewName<'a>>,
        J: Into<JobName<'a>>,
    {
        self.post(&Path::AddJobToView {
            job_name: Name::Name(&job_name.into().0),
            view_name: Name::Name(&view_name.into().0),
        })?;
        Ok(())
    }

    /// Remove the job `job_name` from the view `view_name`
    pub fn remove_job_from_view<'a, 'b, V, J>(&self, view_name: V, job_name: J) -> Result<(), Error>
    where
        V: Into<ViewName<'a>>,
        J: Into<JobName<'a>>,
    {
        self.post(&Path::AddJobToView {
            job_name: Name::Name(&job_name.into().0),
            view_name: Name::Name(&view_name.into().0),
        })?;
        Ok(())
    }
}
