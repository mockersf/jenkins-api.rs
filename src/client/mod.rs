use failure;
use regex::Regex;
use reqwest::header::ContentType;
use reqwest::{Body, Client, RequestBuilder, Response, StatusCode};
use std::fmt::Debug;

mod errors;
pub use self::errors::Error;
mod path;
pub(crate) use self::path::{Name, Path};
mod builder;
pub use self::builder::JenkinsBuilder;
mod csrf;

/// Helper type for error management
pub mod error {
    pub use super::errors::Action;
    pub use super::errors::ExpectedType;
}

#[derive(Debug)]
struct User {
    username: String,
    password: Option<String>,
}

/// Client struct with the methods to query Jenkins
#[derive(Debug)]
pub struct Jenkins {
    url: String,
    client: Client,
    user: Option<User>,
    csrf_enabled: bool,
}

impl Jenkins {
    pub(crate) fn url_api_json(&self, endpoint: &str) -> String {
        format!("{}{}/api/json", self.url, endpoint)
    }

    pub(crate) fn url(&self, endpoint: &str) -> String {
        format!("{}{}", self.url, endpoint)
    }

    fn send(&self, mut request_builder: RequestBuilder) -> Result<Response, failure::Error> {
        let query = request_builder.build()?;
        debug!("sending {} {}", query.method(), query.url());
        Ok(self.client.execute(query)?)
    }

    fn error_for_status(response: Response) -> Result<Response, failure::Error> {
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            warn!("got an error: {}", status);
        }
        Ok(response.error_for_status()?)
    }

    pub(crate) fn get(&self, path: &Path) -> Result<Response, failure::Error> {
        let query = self.client.get(&self.url_api_json(&path.to_string()));
        Ok(Self::error_for_status(self.send(query)?)?)
    }

    pub(crate) fn get_with_params(
        &self,
        path: &Path,
        qps: &[(&str, &str)],
    ) -> Result<Response, failure::Error> {
        let mut query = self.client.get(&self.url_api_json(&path.to_string()));
        query.query(qps);
        Ok(Self::error_for_status(self.send(query)?)?)
    }

    pub(crate) fn post(&self, path: &Path) -> Result<Response, failure::Error> {
        let mut request_builder = self.client.post(&self.url(&path.to_string()));

        self.add_csrf_to_request(&mut request_builder)?;

        Ok(request_builder.send()?.error_for_status()?)
    }

    pub(crate) fn post_with_body<T: Into<Body> + Debug>(
        &self,
        path: &Path,
        body: T,
        qps: &[(&str, &str)],
    ) -> Result<Response, failure::Error> {
        let mut request_builder = self.client.post(&self.url(&path.to_string()));

        self.add_csrf_to_request(&mut request_builder)?;

        request_builder.header(ContentType::form_url_encoded());
        request_builder.query(qps).body(body);
        let mut response = self.send(request_builder)?;

        if response.status() == StatusCode::InternalServerError {
            let body = response.text()?;

            let re = Regex::new(r"java.lang.([a-zA-Z]+): (.*)").unwrap();
            if let Some(captures) = re.captures(&body) {
                match captures.get(1).map(|v| v.as_str()) {
                    Some("IllegalStateException") => {
                        warn!(
                            "got an IllegalState error: {}",
                            captures.get(0).map(|v| v.as_str()).unwrap_or("unspecified")
                        );
                        Err(Error::IllegalState {
                            message: captures
                                .get(2)
                                .map(|v| v.as_str())
                                .unwrap_or("no message")
                                .to_string(),
                        })
                    }
                    Some("IllegalArgumentException") => {
                        warn!(
                            "got an IllegalArgument error: {}",
                            captures.get(0).map(|v| v.as_str()).unwrap_or("unspecified")
                        );
                        Err(Error::IllegalArgument {
                            message: captures
                                .get(2)
                                .map(|v| v.as_str())
                                .unwrap_or("no message")
                                .to_string(),
                        })
                    }
                    Some(_) => {
                        warn!(
                            "got an Unknwon error: {}",
                            captures.get(0).map(|v| v.as_str()).unwrap_or("unspecified")
                        );
                        Ok(())
                    }
                    _ => Ok(()),
                }?;
            }
        }

        Ok(Self::error_for_status(response)?)
    }
}
