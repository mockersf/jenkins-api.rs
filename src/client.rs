use reqwest::header::{Authorization, Basic, Formatter, Header, Headers, Raw};
use reqwest::{Client, Error, RequestBuilder, Response};

use serde::Deserialize;
use urlencoding;

use failure;

use error;

#[derive(Debug)]
struct User {
    username: String,
    password: Option<String>,
}

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

    pub(crate) fn get(&self, path: &Path) -> RequestBuilder {
        self.client.get(&self.url_api_json(&path.to_string()))
    }

    pub(crate) fn post(&self, path: &Path) -> Result<Response, failure::Error> {
        let mut request_builder = self.client.post(&self.url(&path.to_string()));

        if self.csrf_enabled {
            let crumb: Crumb = self.get(&Path::CrumbIssuer)
                .send()?
                .error_for_status()?
                .json()?;
            if crumb.crumb_request_field != Crumb::header_name() {
                return Err(error::Error::InvalidCrumbFieldName {
                    field_name: crumb.crumb_request_field,
                }.into());
            }
            request_builder.header(crumb);
        }

        Ok(request_builder.send()?.error_for_status()?)
    }

    pub(crate) fn url_to_path<'a>(&self, url: &'a str) -> Path<'a> {
        let path = if url.starts_with(&self.url) {
            &url[self.url.len()..]
        } else {
            url
        };
        let first_slash = path.char_indices().filter(|c| c.1 == '/').nth(1).unwrap().0;
        match (
            &path[0..first_slash],
            path.chars().filter(|c| *c == '/').count(),
        ) {
            ("/view", 3) => Path::View {
                name: Name::UrlEncodedName(&path[6..(path.len() - 1)]),
            },
            ("/job", 3) => Path::Job {
                name: Name::UrlEncodedName(&path[5..(path.len() - 1)]),
            },
            ("/job", 4) => {
                let last_slash = path.char_indices()
                    .rev()
                    .filter(|c| c.1 == '/')
                    .nth(1)
                    .unwrap()
                    .0;
                Path::Build {
                    job_name: Name::UrlEncodedName(&path[5..last_slash]),
                    id: path[(last_slash + 1)..(path.len() - 1)].parse().unwrap(),
                }
            }
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
    csrf_enabled: bool,
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
            csrf_enabled: true,
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
            csrf_enabled: self.csrf_enabled,
        })
    }

    pub fn with_user(mut self, login: &str, password: Option<&str>) -> Self {
        self.user = Some(User {
            username: login.to_string(),
            password: password.map(|s| s.to_string()),
        });
        self
    }

    pub fn disable_csrf(mut self) -> Self {
        self.csrf_enabled = false;
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
    JobEnable { name: Name<'a> },
    JobDisable { name: Name<'a> },
    Build { job_name: Name<'a>, id: u32 },
    Raw { path: &'a str },
    CrumbIssuer,
}

impl<'a> ToString for Path<'a> {
    fn to_string(&self) -> String {
        match *self {
            Path::Home => "".to_string(),
            Path::View { ref name } => format!("/view/{}", name.to_string()),
            Path::Job { ref name } => format!("/job/{}", name.to_string()),
            Path::JobEnable { ref name } => format!("/job/{}/enable", name.to_string()),
            Path::JobDisable { ref name } => format!("/job/{}/disable", name.to_string()),
            Path::Build {
                ref job_name,
                ref id,
            } => format!("/job/{}/{}", job_name.to_string(), id),
            Path::Raw { path } => format!("{}", path),
            Path::CrumbIssuer => "/crumbIssuer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Crumb {
    crumb: String,
    crumb_request_field: String,
}
use std::fmt;
use hyper;
impl Header for Crumb {
    fn header_name() -> &'static str {
        "Jenkins-Crumb"
    }

    fn parse_header(_: &Raw) -> Result<Self, hyper::error::Error> {
        unimplemented!();
    }

    fn fmt_header(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.fmt_line(&self.crumb)
    }
}
