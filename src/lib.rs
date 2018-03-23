#![deny(warnings)]

extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate urlencoding;

extern crate failure;

mod client;
pub use client::{Jenkins, JenkinsBuilder};

mod list;
mod jobs;
