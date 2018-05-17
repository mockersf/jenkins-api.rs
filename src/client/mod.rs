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

#[derive(Debug, PartialEq)]
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
        info!("sending {} {}", query.method(), query.url());
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
        self.get_with_params(path, &[("depth", "1")])
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

#[cfg(test)]
mod tests {
    extern crate mockito;

    static JENKINS_URL: &'static str = mockito::SERVER_URL;

    #[test]
    fn can_post_with_body() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL)
            .disable_csrf()
            .build()
            .unwrap();

        let _mock = mockito::mock("POST", "/mypath?").with_body("ok").create();

        let response =
            jenkins_client.post_with_body(&super::Path::Raw { path: "/mypath" }, "body", &[]);

        assert!(response.is_ok());
        assert_eq!(response.unwrap().text().unwrap(), "ok");
    }

    #[test]
    fn can_post_with_body_and_get_error_state() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL)
            .disable_csrf()
            .build()
            .unwrap();

        let _mock = mockito::mock("POST", "/error-IllegalStateException?")
            .with_status(500)
            .with_body("hviqsuvnqsodjfsqjdgo java.lang.IllegalStateException: my error\nvzfjsd")
            .create();

        let response = jenkins_client.post_with_body(
            &super::Path::Raw {
                path: "/error-IllegalStateException",
            },
            "body",
            &[],
        );

        assert!(response.is_err());
        assert_eq!(
            format!("{:?}", response),
            r#"Err(IllegalState { message: "my error" })"#
        );
    }

    #[test]
    fn can_post_with_body_and_get_error_argument() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL)
            .disable_csrf()
            .build()
            .unwrap();

        let _mock = mockito::mock("POST", "/error-IllegalArgumentException?")
            .with_status(500)
            .with_body("hviqsuvnqsodjfsqjdgo java.lang.IllegalArgumentException: my error\nvzfjsd")
            .create();

        let response = jenkins_client.post_with_body(
            &super::Path::Raw {
                path: "/error-IllegalArgumentException",
            },
            "body",
            &[],
        );

        assert!(response.is_err());
        assert_eq!(
            format!("{:?}", response),
            r#"Err(IllegalArgument { message: "my error" })"#
        );
    }

    #[test]
    fn can_post_with_body_and_get_error_new() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL)
            .disable_csrf()
            .build()
            .unwrap();

        let _mock = mockito::mock("POST", "/error-NewException?")
            .with_status(500)
            .with_body("hviqsuvnqsodjfsqjdgo java.lang.NewException: my error\nvzfjsd")
            .create();

        let response = jenkins_client.post_with_body(
            &super::Path::Raw {
                path: "/error-NewException",
            },
            "body",
            &[],
        );

        assert!(response.is_err());
        assert_eq!(
            format!("{:?}", response),
            concat!(
                r#"Err(Error { kind: ServerError(InternalServerError), "#,
                r#"url: Some("http://127.0.0.1:1234/error-NewException?") })"#
            )
        );
    }

    #[test]
    fn can_post_with_query_params() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL)
            .disable_csrf()
            .build()
            .unwrap();

        let mock = mockito::mock("POST", "/mypath?a=1")
            .with_body("ok")
            .create();

        let response = jenkins_client.post_with_body(
            &super::Path::Raw { path: "/mypath" },
            "body",
            &[("a", "1")],
        );

        assert!(response.is_ok());
        assert_eq!(response.unwrap().text().unwrap(), "ok");
        mock.assert()
    }

}
