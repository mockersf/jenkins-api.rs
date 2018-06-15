use std::marker::PhantomData;

use failure::Error;
use serde;
use serde_json;

use helpers::Class;

use super::JobBuilder;
use action::CommonAction;
use build::{CommonBuild, ShortBuild};
use client::{self, Name, Path};
use property::CommonProperty;
use queue::ShortQueueItem;
use view::ViewName;
use Jenkins;

/// Ball Color corresponding to a `BuildStatus`
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BallColor {
    /// Success
    Blue,
    /// Success, and build is on-going
    BlueAnime,
    /// Unstable
    Yellow,
    /// Unstable, and build is on-going
    YellowAnime,
    /// Failure
    Red,
    /// Failure, and build is on-going
    RedAnime,
    /// Catch-all for disabled, aborted, not yet build
    Grey,
    /// Catch-all for disabled, aborted, not yet build, and build is on-going
    GreyAnime,
    /// Disabled
    Disabled,
    /// Disabled, and build is on-going
    DisabledAnime,
    /// Aborted
    Aborted,
    ///Aborted, and build is on-going
    AbortedAnime,
    /// Not Build
    #[serde(rename = "notbuilt")]
    NotBuilt,
    /// Not Build, and build is on-going
    #[serde(rename = "notbuilt_anime")]
    NotBuiltAnime,
}

/// Health Report of a `Job`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    /// Description of the `HealthReport`
    pub description: String,
    /// Icon name
    pub icon_class_name: String,
    /// Icon url
    pub icon_url: String,
    /// Score of the `Job`
    pub score: u16,
}

/// Short Job that is used in lists and links from other structs
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortJob<T: Job = CommonJob> {
    /// Name of the job
    pub name: String,
    /// URL for the job
    pub url: String,
    /// Ball Color for the status of the job
    pub color: BallColor,
    #[serde(flatten)]
    pub(crate) other_fields: Option<serde_json::Value>,

    #[serde(skip)]
    job_type: PhantomData<T>,
}
impl<T> ShortJob<T>
where
    T: Job,
    for<'de> T: serde::Deserialize<'de>,
{
    /// Get the full details of a `Job` matching the `ShortJob`
    pub fn get_full_job(&self, jenkins_client: &Jenkins) -> Result<T, Error> {
        let path = jenkins_client.url_to_path(&self.url);
        if let Path::Job { .. } = path {
            Ok(jenkins_client.get(&path)?.json()?)
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url.clone(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }
}

/// Helper type to act on a job
#[derive(Debug)]
pub struct JobName<'a>(pub &'a str);
impl<'a> From<&'a str> for JobName<'a> {
    fn from(v: &'a str) -> JobName<'a> {
        JobName(v)
    }
}
impl<'a> From<&'a String> for JobName<'a> {
    fn from(v: &'a String) -> JobName<'a> {
        JobName(v)
    }
}
impl<'a> From<&'a ShortJob> for JobName<'a> {
    fn from(v: &'a ShortJob) -> JobName<'a> {
        JobName(&v.name)
    }
}
impl<'a, T: Job> From<&'a T> for JobName<'a> {
    fn from(v: &'a T) -> JobName<'a> {
        JobName(v.name())
    }
}

/// Trait implemented by specializations of `Job` and providing common methods
pub trait Job {
    /// get the url of a `Job`
    fn url(&self) -> &str;
    /// Get the name of the project
    fn name(&self) -> &str;

    /// Enable a `Job`. It may need to be refreshed as it may have been updated
    fn enable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::JobEnable { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }

    /// Disable a `Job`. It may need to be refreshed as it may have been updated
    fn disable(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::JobDisable { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }

    /// Add this job to the view `view_name`
    fn add_to_view<'a, V>(&self, jenkins_client: &Jenkins, view_name: V) -> Result<(), Error>
    where
        V: Into<ViewName<'a>>,
    {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::AddJobToView {
                job_name: name,
                view_name: Name::Name(view_name.into().0),
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }

    /// Remove this job from the view `view_name`
    fn remove_from_view<'a, V>(&self, jenkins_client: &Jenkins, view_name: V) -> Result<(), Error>
    where
        V: Into<ViewName<'a>>,
    {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::RemoveJobFromView {
                job_name: name,
                view_name: Name::Name(view_name.into().0),
            })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }
}

macro_rules! job_build_with_common_fields_and_impl {
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
        job_build_with_common_fields_and_impl! {
            $(#[$attr])*
            pub struct $name<BuildType = CommonBuild> {
                $(
                    $(#[$field_attr])*
                    pub $field: $field_type,
                )*
                $(private_fields {
                    $(
                        $(#[$private_field_attr])*
                        $private_field: $private_field_type
                    ),*
                })*
            }
        }
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident<BuildType = $build_type:ty> {
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
            /// Name of the job
            pub name: String,
            /// Display Name of the job
            pub display_name: String,
            /// Full Display Name of the job
            pub full_display_name: String,
            /// Full Name of the job
            pub full_name: String,
            /// Display Name of the job
            pub display_name_or_null: Option<String>,
            /// URL for the job
            pub url: String,
            /// Ball Color for the status of the job
            pub color: BallColor,
            /// Is the job buildable?
            pub buildable: bool,
            /// Are dependencies kept for this job?
            pub keep_dependencies: bool,
            /// Next build number
            pub next_build_number: u32,
            /// Is this job currently in build queue
            pub in_queue: bool,
            /// Actions of a job
            pub actions: Vec<Option<CommonAction>>,
            /// Link to the last build
            pub last_build: Option<ShortBuild<$build_type>>,
            /// Link to the first build
            pub first_build: Option<ShortBuild<$build_type>>,
            /// Link to the last stable build
            pub last_stable_build: Option<ShortBuild<$build_type>>,
            /// Link to the last unstable build
            pub last_unstable_build: Option<ShortBuild<$build_type>>,
            /// Link to the last successful build
            pub last_successful_build: Option<ShortBuild<$build_type>>,
            /// Link to the last unsucressful build
            pub last_unsuccessful_build: Option<ShortBuild<$build_type>>,
            /// Link to the last complete build
            pub last_completed_build: Option<ShortBuild<$build_type>>,
            /// Link to the last failed build
            pub last_failed_build: Option<ShortBuild<$build_type>>,
            /// List of builds of the job
            pub builds: Vec<ShortBuild>,
            /// HealthReport of the job
            pub health_report: Vec<HealthReport>,
            /// Queue item of this job if it's waiting
            pub queue_item: Option<ShortQueueItem>,
            /// Properties of the job
            property: Vec<CommonProperty>,
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
            $($(
                $(#[$private_field_attr])*
                $private_field: $private_field_type,
            )*)*
        }
        impl Job for $name {
            fn url(&self) -> &str {
                &self.url
            }

            fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

job_build_with_common_fields_and_impl!(
    /// A Jenkins `Job`
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CommonJob {
        /// _class provided by Jenkins
        #[serde(rename = "_class")]
        pub class: Option<String>,

        private_fields {
            #[serde(flatten)]
            other_fields: serde_json::Value,
        }
    }
);
specialize!(CommonJob => Job);

impl CommonJob {}

/// Common trait for jobs that can be build
pub trait BuildableJob: Job + Sized {
    /// Build this job
    fn build(&self, jenkins_client: &Jenkins) -> Result<ShortQueueItem, Error> {
        self.builder(jenkins_client)?.send()
    }

    /// Create a `JobBuilder` to setup a build of a `Job`
    fn builder<'a, 'b, 'c, 'd>(
        &'a self,
        jenkins_client: &'b Jenkins,
    ) -> Result<JobBuilder<'a, 'b, 'c, 'd>, Error> {
        JobBuilder::new(self, jenkins_client)
    }
}

/// Common trait for jobs that can poll a SCM
pub trait SCMPollable: Job + Sized {
    /// Poll configured SCM for changes
    fn poll_scm(&self, jenkins_client: &Jenkins) -> Result<(), Error> {
        let path = jenkins_client.url_to_path(&self.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            jenkins_client.post(&Path::PollSCMJob { name })?;
            Ok(())
        } else {
            Err(client::Error::InvalidUrl {
                url: self.url().to_string(),
                expected: client::error::ExpectedType::Job,
            }.into())
        }
    }
}
