#![deny(warnings, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications, missing_docs)]

//! Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)
//!
//! # Example
//!
//! ```rust
//! extern crate failure;
//!
//! extern crate jenkins_api;
//!
//! use jenkins_api::JenkinsBuilder;
//!
//! fn main() -> Result<(), failure::Error> {
//!     let jenkins = JenkinsBuilder::new("http://localhost:8080")
//!         .with_user("user", Some("password"))
//!         .build()?;
//!
//!     let job = jenkins.get_job("job name")?;
//!     let build = job.last_build()?.as_ref().unwrap().get_full_build(&jenkins)?;
//!
//!     println!(
//!         "last build for job {} at {} was {:?}",
//!         job.name()?, build.timestamp()?, build.result()?
//!     );
//!     Ok(())
//! }
//! ```
//!

extern crate hyper;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

extern crate urlencoding;

#[macro_use]
extern crate failure;
extern crate regex;

#[macro_use]
extern crate log;

mod client;
pub use client::{error, Error, Jenkins, JenkinsBuilder};

#[macro_use]
mod helpers;

mod view;
pub use view::*;
mod job;
pub use job::*;
pub mod action;
mod build;
pub mod job_builder;
pub use build::*;
mod queue;
pub use queue::*;
mod user;
pub use user::*;
