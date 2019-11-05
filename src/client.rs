//! Helpers to build advanced queries

use serde::{self, Deserialize};

use crate::client_internals::path::{Name, Path as PrivatePath};
use crate::client_internals::InternalAdvancedQueryParams;

// pub use client_internals::path::Name;
pub use crate::client_internals::AdvancedQuery;
pub use crate::client_internals::{error, Error};
pub use crate::client_internals::{TreeBuilder, TreeQueryParam};

use crate::build;

/// Path to an object in Jenkins
#[derive(Debug, PartialEq)]
pub enum Path<'a> {
    /// Path to the home
    Home,
    /// Path to a view
    View {
        /// The view name
        name: &'a str,
    },
    /// Path to a job
    Job {
        /// The job name
        name: &'a str,
        /// The job configuration
        configuration: Option<&'a str>,
    },
    /// Path to a job build
    Build {
        /// The job name
        job_name: &'a str,
        /// The build number
        number: build::BuildNumber,
        /// The build configuration
        configuration: Option<&'a str>,
    },
    /// Path to the Jenkins queue
    Queue,
    /// Path to an item in the queue
    QueueItem {
        /// The item id
        id: i32,
    },
    /// Path to a build's maven artifacts
    MavenArtifactRecord {
        /// The job name
        job_name: &'a str,
        /// The build number
        number: build::BuildNumber,
        /// The build configuration
        configuration: Option<&'a str>,
    },
    /// Path to the computers linked to Jenkins
    Computers,
    /// Path to a computer
    Computer {
        /// The computer name
        name: &'a str,
    },
    /// Unknown path
    Raw {
        /// The path itself
        path: &'a str,
    },
}

impl<'a> Into<PrivatePath<'a>> for Path<'a> {
    fn into(self) -> PrivatePath<'a> {
        match self {
            Path::Home => PrivatePath::Home,
            Path::View { name } => PrivatePath::View {
                name: Name::Name(name),
            },
            Path::Job {
                name,
                configuration,
            } => PrivatePath::Job {
                name: Name::Name(name),
                configuration: configuration.map(|v| Name::Name(v)),
            },
            Path::Build {
                job_name,
                number,
                configuration,
            } => PrivatePath::Build {
                job_name: Name::Name(job_name),
                number,
                configuration: configuration.map(|v| Name::Name(v)),
            },
            Path::Queue => PrivatePath::Queue,
            Path::QueueItem { id } => PrivatePath::QueueItem { id },
            Path::MavenArtifactRecord {
                job_name,
                number,
                configuration,
            } => PrivatePath::MavenArtifactRecord {
                job_name: Name::Name(job_name),
                number,
                configuration: configuration.map(|v| Name::Name(v)),
            },
            Path::Computers => PrivatePath::Computers,
            Path::Computer { name } => PrivatePath::Computer {
                name: Name::Name(name),
            },
            Path::Raw { path } => PrivatePath::Raw { path },
        }
    }
}

impl super::Jenkins {
    /// Get a `Path` from Jenkins, specifying the depth or tree parameters
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde;
    /// #
    /// # extern crate jenkins_api;
    /// #
    /// # use jenkins_api::JenkinsBuilder;
    /// #
    /// #[derive(Deserialize)]
    /// #[serde(rename_all = "camelCase")]
    /// struct LastBuild {
    ///     number: u32,
    ///     duration: u32,
    ///     result: String,
    /// }
    /// #[derive(Deserialize)]
    /// #[serde(rename_all = "camelCase")]
    /// struct LastBuildOfJob {
    ///     display_name: String,
    ///     last_build: LastBuild,
    /// }
    ///
    /// # fn main() -> Result<(), failure::Error> {
    /// #    let jenkins = JenkinsBuilder::new("http://localhost:8080")
    /// #        .with_user("user", Some("password"))
    /// #        .build()?;
    /// let _: LastBuildOfJob = jenkins.get_object_as(
    ///     jenkins_api::client::Path::Job {
    ///         name: "job name",
    ///         configuration: None,
    ///     },
    ///     jenkins_api::client::TreeBuilder::new()
    ///         .with_field("displayName")
    ///         .with_field(
    ///             jenkins_api::client::TreeBuilder::object("lastBuild")
    ///                 .with_subfield("number")
    ///                 .with_subfield("duration")
    ///                 .with_subfield("result"),
    ///         )
    ///         .build(),
    /// )?;
    /// #    Ok(())
    /// # }
    /// ```
    ///
    pub fn get_object_as<Q, T>(
        &self,
        object: Path,
        parameters: Q,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        Q: Into<Option<AdvancedQuery>>,
        for<'de> T: Deserialize<'de>,
    {
        Ok(self
            .get_with_params(
                &object.into(),
                parameters.into().map(InternalAdvancedQueryParams::from),
            )?
            .json()?)
    }
}
