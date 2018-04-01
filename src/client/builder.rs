use reqwest::header::{Authorization, Basic, Headers};
use reqwest::{Client, Error};

use super::{Jenkins, User};

/// Builder for Jenkins client
///
/// ```rust
///# extern crate jenkins_api;
///#
///# use jenkins_api::JenkinsBuilder;
///#
///# fn main() {
///     let jenkins = JenkinsBuilder::new("http://localhost:8080")
///         .with_user("user", Some("password"))
///         .build()
///         .unwrap();
///# }
/// ```
#[derive(Debug)]
pub struct JenkinsBuilder {
    url: String,
    user: Option<User>,
    csrf_enabled: bool,
}

impl JenkinsBuilder {
    /// Create a new builder with Jenkins url
    pub fn new(url: &str) -> Self {
        JenkinsBuilder {
            url: {
                let last: String = url.chars().rev().take(1).collect();
                match last.as_str() {
                    "/" => url[0..(url.len() - 1)].to_string(),
                    _ => url.to_string(),
                }
            },
            user: None,
            csrf_enabled: true,
        }
    }

    /// Build the Jenkins client
    pub fn build(self) -> Result<Jenkins, Error> {
        let mut headers = Headers::new();

        if let Some(ref user) = self.user {
            headers.set(Authorization(Basic {
                username: user.username.clone(),
                password: user.password.clone(),
            }));
        }

        Ok(Jenkins {
            url: self.url,
            client: Client::builder().default_headers(headers).build()?,
            user: self.user,
            csrf_enabled: self.csrf_enabled,
        })
    }

    /// Specify the user to use for authorizing queries
    pub fn with_user(mut self, login: &str, password: Option<&str>) -> Self {
        self.user = Some(User {
            username: login.to_string(),
            password: password.map(|s| s.to_string()),
        });
        self
    }

    /// Disable CSRF in crumbs used for post queries
    pub fn disable_csrf(mut self) -> Self {
        self.csrf_enabled = false;
        self
    }
}
