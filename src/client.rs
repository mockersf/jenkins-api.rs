use reqwest::header::{Authorization, Basic, Headers};
use reqwest::{Client, Error, RequestBuilder};

use serde::Deserialize;
use urlencoding;

use failure;

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
        self.client.get(&self.url_api_json(&path.to_string()))
    }

    pub(crate) fn url_to_path<'a>(&self, url: &'a str) -> Path<'a> {
        let path = if url.starts_with(&self.url) {
            &url[self.url.len()..]
        } else {
            url
        };
        match (&path[0..4], path.chars().filter(|c| *c == '/').count()) {
            ("/vie", 3) => Path::View {
                name: Name::UrlEncodedName(&path[6..(path.len() - 1)]),
            },
            (_, _) => Path::Raw { path },
        }
    }

    pub(crate) fn get_from_url<T>(&self, url: &str) -> Result<T, failure::Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        Ok(self.get(&self.url_to_path(url))
            .send()?
            .error_for_status()?
            .json()?)
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

#[derive(Debug)]
pub(crate) enum Name<'a> {
    Name(&'a str),
    UrlEncodedName(&'a str),
}

impl<'a> ToString for Name<'a> {
    fn to_string(&self) -> String {
        match *self {
            Name::Name(name) => urlencoding::encode(name),
            Name::UrlEncodedName(name) => name.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Path<'a> {
    Home,
    View { name: Name<'a> },
    Job { name: Name<'a> },
    Build { job_name: Name<'a>, id: u32 },
    Raw { path: &'a str },
}

impl<'a> ToString for Path<'a> {
    fn to_string(&self) -> String {
        match *self {
            Path::Home => "".to_string(),
            Path::View { ref name } => format!("/view/{}", name.to_string()),
            Path::Job { ref name } => format!("/job/{}", name.to_string()),
            Path::Build {
                ref job_name,
                ref id,
            } => format!("/job/{}/{}", job_name.to_string(), id),
            Path::Raw { path } => format!("{}", path),
        }
    }
}
