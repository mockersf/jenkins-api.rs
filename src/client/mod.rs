use reqwest::{Body, Client, Response, StatusCode};
use reqwest::header::ContentType;
use std::fmt::Debug;
use failure;
use regex::Regex;

mod error;
pub use self::error::Error;
mod path;
pub(crate) use self::path::{Name, Path};
mod builder;
pub use self::builder::JenkinsBuilder;
mod csrf;

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

    pub(crate) fn get(&self, path: &Path) -> Result<Response, failure::Error> {
        Ok(self.client
            .get(&self.url_api_json(&path.to_string()))
            .send()?
            .error_for_status()?)
    }

    pub(crate) fn get_with_params(
        &self,
        path: &Path,
        qps: &[(&str, &str)],
    ) -> Result<Response, failure::Error> {
        Ok(self.client
            .get(&self.url_api_json(&path.to_string()))
            .query(qps)
            .send()?
            .error_for_status()?)
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
        let mut response = request_builder.query(qps).body(body).send()?;

        if response.status() == StatusCode::InternalServerError {
            let body = response.text()?;

            let re = Regex::new(r"java.lang.([a-zA-Z]+): (.*)").unwrap();
            if let Some(captures) = re.captures(&body) {
                match captures.get(1).map(|v| v.as_str()) {
                    Some("IllegalStateException") => Err(Error::IllegalState {
                        message: captures
                            .get(2)
                            .map(|v| v.as_str())
                            .unwrap_or("no message")
                            .to_string(),
                    }),
                    Some("IllegalArgumentException") => Err(Error::IllegalArgument {
                        message: captures
                            .get(2)
                            .map(|v| v.as_str())
                            .unwrap_or("no message")
                            .to_string(),
                    }),
                    _ => Ok(()),
                }?;
            }
        }

        Ok(response.error_for_status()?)
    }
}
