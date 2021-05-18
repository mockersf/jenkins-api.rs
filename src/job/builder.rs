//! Helper to build a job

use reqwest::header::LOCATION;

use serde::{self, Serialize};

use crate::client::{self, Result};
use crate::client_internals::{Name, Path};
use crate::job::{Job, JobName};
use crate::queue::ShortQueueItem;
use crate::Jenkins;

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
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new<T>(job: &'a T, jenkins_client: &'b Jenkins) -> Result<Self>
    where
        T: Job,
    {
        let path = jenkins_client.url_to_path(&job.url());
        if let Path::Job {
            name,
            configuration: None,
        } = path
        {
            return Ok(JobBuilder {
                job_name: name,
                jenkins_client,
                delay: None,
                cause: None,
                token: None,
                parameters: None,
            });
        } else if let Path::InFolder {
            folder_name: _folder_name,
            path: sub_path,
        } = path
        {
            if let Path::Job {
                name,
                configuration: None,
            } = sub_path.as_ref()
            {
                return Ok(JobBuilder {
                    job_name: name.clone(),
                    jenkins_client,
                    delay: None,
                    cause: None,
                    token: None,
                    parameters: None,
                });
            }
        }
        Err(client::Error::InvalidUrl {
            url: job.url().to_string(),
            expected: client::error::ExpectedType::Job,
        }
        .into())
    }

    pub(crate) fn new_from_job_name<J>(
        name: J,
        jenkins_client: &'b Jenkins,
        name_encoded: bool,
    ) -> Result<Self>
    where
        J: Into<JobName<'a>>,
    {
        let job_name = if !name_encoded {
            Name::Name(name.into().0)
        } else {
            Name::UrlEncodedName(name.into().0)
        };
        Ok(JobBuilder {
            job_name,
            jenkins_client,
            delay: None,
            cause: None,
            token: None,
            parameters: None,
        })
    }

    /// Trigger the build
    pub fn send(self) -> Result<ShortQueueItem> {
        let response = match (self.token, self.parameters) {
            (Some(token), None) => {
                let bound_cause = self.cause.unwrap_or("");
                let bound_delay = format!("{}", self.delay.unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                qps.push(("token", &token));
                if self.cause.is_some() {
                    qps.push(("cause", &bound_cause));
                }
                if self.delay.is_some() {
                    qps.push(("delay", &bound_delay));
                }

                self.jenkins_client.get_with_params(
                    &Path::BuildJob {
                        name: self.job_name,
                    },
                    &qps,
                )?
            }
            (Some(token), Some(parameters)) => {
                let bound_delay = format!("{}", self.delay.unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                if self.delay.is_some() {
                    qps.push(("delay", &bound_delay));
                }
                self.jenkins_client.post_with_body(
                    &Path::BuildJobWithParameters {
                        name: self.job_name,
                    },
                    format!("token={}&{}", token, parameters),
                    &qps,
                )?
            }
            (None, None) => {
                let bound_delay = format!("{}", self.delay.unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                if self.delay.is_some() {
                    qps.push(("delay", &bound_delay));
                }
                self.jenkins_client.post_with_body(
                    &Path::BuildJob {
                        name: self.job_name,
                    },
                    "",
                    &qps,
                )?
            }
            (None, Some(parameters)) => {
                let bound_delay = format!("{}", self.delay.unwrap_or(0));
                let mut qps: Vec<(&str, &str)> = Vec::new();
                if self.delay.is_some() {
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
        if let Some(location) = response.headers().get(LOCATION) {
            Ok(ShortQueueItem {
                url: location.to_str().unwrap().to_string(),
                extra_fields: None,
            })
        } else {
            Err(client::Error::InvalidUrl {
                url: "".to_string(),
                expected: client::error::ExpectedType::QueueItem,
            }
            .into())
        }
    }

    /// Add a delay before the job will be built
    pub fn with_delay(mut self, delay_sec: u32) -> Self {
        self.delay = Some(delay_sec);
        self
    }

    /// Trigger the build remotely with a token and a cause
    pub fn remotely_with_token_and_cause(
        mut self,
        token: &'d str,
        cause: Option<&'c str>,
    ) -> Result<Self> {
        self.token = Some(token);
        self.cause = cause;
        Ok(self)
    }

    /// Build with parameters
    ///
    /// Supported parameters type: Boolean, Choice, Multi-line string, Password, Run, String
    ///
    /// Unsupported parameters type: File, Credentials
    /// # Errors
    /// If used on a `Job` without parameters, sending this build will return an
    /// [`Error::IllegalState`](../enum.Error.html#variant.IllegalState)
    ///
    /// If used with invalid parameters type / value, sending this build will return an
    /// [`Error::IllegalArgument`](../enum.Error.html#variant.IllegalArgument)
    ///
    /// This methods will return an error if serializing `parameters` fails.
    pub fn with_parameters<T: Serialize>(mut self, parameters: &T) -> Result<Self> {
        if self.token.is_some() {
            return Err(client::Error::UnsupportedBuildConfiguration.into());
        }
        self.parameters = Some(serde_urlencoded::to_string(parameters)?);
        Ok(self)
    }
}
