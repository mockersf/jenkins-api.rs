use failure::Error;
use serde::Deserializer;

use Jenkins;
use client::{self, Name, Path};
use job::ShortJob;

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

tagged_enum_or_default!{
    /// Short View that is used in lists and links from other structs
    pub enum ShortView {
        /// A view listing jobs
        ListView (_class = "hudson.model.ListView") {
            /// Name of the view
            name: String,
            /// URL for the view
            url: String,
        },
        /// A view listing all jobs
        AllView (_class = "hudson.model.AllView") {
            /// Name of the view
            name: String,
            /// URL for the view
            url: String,
        },
    }
}

macro_rules! shortview_common_fields_dispatch {
    ($field:ident -> $return:ty) => {
        pub(crate) fn $field(&self) -> Result<$return, Error> {
            match self {
                &ShortView::ListView { ref $field, .. } => Ok($field),
                &ShortView::AllView { ref $field, .. } => Ok($field),
                x @ &ShortView::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::ShortView,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &ShortView::ListView { $field, .. } => Ok($field),
                &ShortView::AllView { $field, .. } => Ok($field),
                x @ &ShortView::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::ShortView,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub ref $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &ShortView::ListView { ref $field, .. } => Ok($field),
                &ShortView::AllView { ref $field, .. } => Ok($field),
                x @ &ShortView::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::ShortView,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
}

impl ShortView {
    shortview_common_fields_dispatch!(
        /// Get the name of the view
        pub ref name -> &str
    );

    /// Get the full details of a `View` matching the `ShortView`
    pub fn get_full_view(&self, jenkins_client: &Jenkins) -> Result<View, Error> {
        match self {
            &ShortView::ListView { ref url, .. } => {
                let path = jenkins_client.url_to_path(url);
                if let Path::View { .. } = path {
                    Ok(jenkins_client.get(&path)?.json()?)
                } else {
                    Err(client::Error::InvalidUrl {
                        url: url.to_string(),
                        expected: client::error::ExpectedType::View,
                    }.into())
                }
            }
            x @ &ShortView::AllView { .. } | x @ &ShortView::Unknown { .. } => {
                Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::ShortView,
                    action: client::error::Action::GetLinkedItem(client::error::ExpectedType::View),
                    variant_name: x.variant_name().to_string(),
                }.into())
            }
        }
    }
}

tagged_enum_or_default!{
    /// A Jenkins `View` with a list of `ShortJob`
    pub enum View {
        /// A view listing jobs
        ListView (_class = "hudson.model.ListView") {
            /// Description of the view
            description: Option<String>,
            /// Name of the view
            name: String,
            /// URL for the view
            url: String,
            /// List of jobs in the view
            jobs: Vec<ShortJob>,
        },
    }
}
macro_rules! view_common_fields_dispatch {
    ($field:ident -> $return:ty) => {
        pub(crate) fn $field(&self) -> Result<$return, Error> {
            match self {
                &View::ListView { ref $field, .. } => Ok($field),
                x @ &View::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::View,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &View::ListView { $field, .. } => Ok($field),
                x @ &View::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::View,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
    ($(#[$attr:meta])* pub ref $field:ident -> $return:ty) => {
        $(#[$attr])*
        pub fn $field(&self) -> Result<$return, Error> {
            match self {
                &View::ListView { ref $field, .. } => Ok($field),
                x @ &View::Unknown { .. } => Err(client::Error::InvalidObjectType {
                    object_type: client::error::ExpectedType::View,
                    action: client::error::Action::GetField(stringify!($field)),
                    variant_name: x.variant_name().to_string(),
                }.into()),
            }
        }
    };
}
impl View {
    view_common_fields_dispatch!(url -> &str);
    view_common_fields_dispatch!(
        /// Get the name of the view
        pub ref name -> &str
    );
    view_common_fields_dispatch!(
        /// Get the jobs from the view
        pub ref jobs -> &Vec<ShortJob>
    );

    /// Add the job `job_name` to this view
    pub fn add_job(&self, jenkins_client: &Jenkins, job_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url()?);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::AddJobToView {
                job_name: Name::Name(job_name),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url()?.to_string(),
                expected: client::error::ExpectedType::View,
            }.into())
        }
    }

    /// Remove the job `job_name` from this view
    pub fn remove_job(&self, jenkins_client: &Jenkins, job_name: &str) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url()?);
        if let Path::View { name } = path {
            jenkins_client.post(&Path::RemoveJobFromView {
                job_name: Name::Name(job_name),
                view_name: name,
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url()?.to_string(),
                expected: client::error::ExpectedType::View,
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

#[cfg(test)]
mod tests {
    use super::*;

    static JENKINS_URL: &'static str = "http://none:8080";

    #[test]
    fn get_unknown_view() {
        let _jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let view = ::serde_json::from_str::<View>(r#"{}"#);
        assert!(view.is_ok());
        let view_ok = view.unwrap();

        assert!(view_ok.name().is_err());
        assert!(view_ok.jobs().is_err());
    }
}
