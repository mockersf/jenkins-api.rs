//! Helper to build a job

use failure::Error;

use reqwest::header::Location;

use serde_urlencoded;
use serde;

use Jenkins;
use job::Job;
use queue::ShortQueueItem;
use client::{self, Name, Path};

/// Helper to build a job
#[derive(Debug)]
pub struct JobBuilder<'a, 'b, 'c, 'd> {
    job_name: Name<'a>,
    jenkins_client: &'b Jenkins,
    delay: Option<u32>,
    cause: Option<&'c str>,
    token: Option<&'d str>,
    parameters: Option<String>,
}

impl<'a, 'b, 'c, 'd> JobBuilder<'a, 'b, 'c, 'd> {
    pub(crate) fn new(job: &'a Job, jenkins_client: &'b Jenkins) -> Result<Self, Error> {
        let path = jenkins_client.url_to_path(&job.url);
        if let Path::Job { name } = path {
            Ok(JobBuilder {
                job_name: name,
                jenkins_client,
                delay: None,
                cause: None,
                token: None,
                parameters: None,
            })
        } else {
            Err(client::Error::InvalidUrl {
                url: job.url.clone(),
                expected: "Job".to_string(),
            }.into())
        }
    }

    pub(crate) fn new_from_job_name(
        name: &'a str,
        jenkins_client: &'b Jenkins,
    ) -> Result<Self, Error> {
        Ok(JobBuilder {
            job_name: client::Name::Name(name),
            jenkins_client,
            delay: None,
            cause: None,
            token: None,
            parameters: None,
        })
    }

    /// Trigger the build
    pub fn send(self) -> Result<ShortQueueItem, Error> {
        let response = match (self.token, self.parameters) {
            (Some(token), None) => {
                let bound_cause = self.cause.clone().unwrap_or("");
                let bound_delay = format!("{}", self.delay.clone().unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                qps.push(("token", &token));
                if let Some(_) = self.cause {
                    qps.push(("cause", &bound_cause));
                }
                if let Some(_) = self.delay {
                    qps.push(("delay", &bound_delay));
                }

                self.jenkins_client.get_with_params(
                    &Path::BuildJob {
                        name: self.job_name,
                    },
                    &qps,
                )?
            }
            (Some(_token), Some(_parameters)) => unimplemented!(),
            (None, None) => self.jenkins_client.post(&Path::BuildJob {
                name: self.job_name,
            })?,
            (None, Some(parameters)) => {
                let bound_delay = format!("{}", self.delay.clone().unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                if let Some(_) = self.delay {
                    qps.push(("delay", &bound_delay));
                }
                self.jenkins_client.post_with_body(
                    &Path::BuildJobWithParameters {
                        name: self.job_name,
                    },
                    parameters,
                    &qps,
                )?
            }
        };
        if let Some(location) = response.headers().get::<Location>() {
            Ok(ShortQueueItem {
                url: location.lines().next().unwrap().to_string(),
            })
        } else {
            Err(client::Error::InvalidUrl {
                url: "".to_string(),
                expected: "ShortQueueItem".to_string(),
            }.into())
        }
    }

    /// Add a delay before the job will be built
    pub fn with_delay(mut self, delay_sec: u32) -> Self {
        self.delay = Some(delay_sec);
        self
    }

    /// Trigger the build remotely with a token and a cause
    pub fn remotely_with_token_and_cause(mut self, token: &'d str, cause: Option<&'c str>) -> Self {
        self.token = Some(token);
        self.cause = cause;
        self
    }

    /// Build with parameters
    /// Supported parameters type: Boolean, Choice, Multi-line string, Password, Run, String
    /// # Errors
    /// If used on a `Job` without parameters, will return an [`Error::IllegalState`](../enum.Error.html#variant.IllegalState)
    ///
    /// If used with invalid parameters type / value, will return an [`Error::IllegalArgument`](../enum.Error.html#variant.IllegalArgument)
    pub fn with_parameters<T: serde::Serialize>(mut self, parameters: &T) -> Result<Self, Error> {
        self.parameters = Some(serde_urlencoded::to_string(parameters)?);
        Ok(self)
    }
}
