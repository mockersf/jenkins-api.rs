use reqwest::header::{Authorization, Basic, Headers};
use reqwest::{Client, Error, RequestBuilder};

use urlencoding;

#[derive(Debug)]
pub(crate) struct User {
    pub(crate) username: String,
    password: Option<String>,
}

#[derive(Debug)]
pub struct Jenkins {
    url: String,
    client: Client,
    pub(crate) user: Option<User>,
}

impl Jenkins {
    pub(crate) fn url_api_json(&self, endpoint: &str) -> String {
        format!("{}{}/api/json", self.url, endpoint)
    }

    pub(crate) fn get(&self, path: &Path) -> RequestBuilder {
        self.client.get(&self.url_api_json(&Path::to_string(path)))
    }
}

pub struct JenkinsBuilder {
    url: String,
    user: Option<User>,
}

impl JenkinsBuilder {
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
        }
    }

    pub fn build(self) -> Result<Jenkins, Error> {
        let mut headers = Headers::new();

        if let &Some(ref user) = &self.user {
            headers.set(Authorization(Basic {
                username: user.username.clone(),
                password: user.password.clone(),
            }));
        }

        Ok(Jenkins {
            url: self.url,
            client: Client::builder().default_headers(headers).build()?,
            user: self.user,
        })
    }

    pub fn with_user(mut self, login: &str, password: Option<&str>) -> Self {
        self.user = Some(User {
            username: login.to_string(),
            password: password.map(|s| s.to_string()),
        });
        self
    }
}

pub(crate) enum Path<'a> {
    Home,
    View { name: &'a str },
    Job { name: &'a str },
}

impl<'a> ToString for Path<'a> {
    fn to_string(&self) -> String {
        match self {
            &Path::Home => "".to_string(),
            &Path::View { ref name } => format!("/view/{}", urlencoding::encode(name)),
            &Path::Job { ref name } => format!("/job/{}", urlencoding::encode(name)),
        }
    }
}
