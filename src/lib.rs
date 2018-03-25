#![deny(warnings)]

//! Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)
//!
//! # Example
//!
//! ```rust
//! extern crate jenkins_api;
//!
//! use jenkins_api::JenkinsBuilder;
//!
//! fn main() {
//!     let jenkins = JenkinsBuilder::new("http://localhost:8080")
//!         .with_user("user", Some("password"))
//!         .build()
//!         .unwrap();
//!
//!     let job = jenkins.get_job("job name").unwrap();
//!     let build = job.last_build.unwrap().get_full_build(&jenkins).unwrap();
//!
//!     println!(
//!         "last build for job {} at {} was {:?}",
//!         job.name, build.timestamp, build.result
//!     );
//! }
//! ```
//!

extern crate hyper;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate urlencoding;

#[macro_use]
extern crate failure;

mod client;
pub use client::{Error, Jenkins, JenkinsBuilder};

mod view;
pub use view::*;
mod job;
pub use job::*;
mod build;
pub use build::*;
mod queue;
pub use queue::*;
