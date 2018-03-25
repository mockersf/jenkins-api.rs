use reqwest::{Client, Response};

use failure;

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

    pub(crate) fn post(&self, path: &Path) -> Result<Response, failure::Error> {
        let mut request_builder = self.client.post(&self.url(&path.to_string()));

        self.add_csrf_to_request(&mut request_builder)?;

        Ok(request_builder.send()?.error_for_status()?)
    }
}
