use failure;
use hyper;
use std::fmt;

use reqwest::header::{Formatter, Header, Raw};
use reqwest::RequestBuilder;

use super::{errors, Jenkins, Path};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Crumb {
    crumb: String,
    crumb_request_field: String,
}

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

impl Jenkins {
    pub(crate) fn add_csrf_to_request(
        &self,
        request_builder: &mut RequestBuilder,
    ) -> Result<(), failure::Error> {
        if self.csrf_enabled {
            let _ = request_builder.header(self.get_csrf()?);
        }
        Ok(())
    }

    pub(crate) fn get_csrf(&self) -> Result<Crumb, failure::Error> {
        let crumb: Crumb = self.get(&Path::CrumbIssuer)?.json()?;
        if crumb.crumb_request_field != Crumb::header_name() {
            return Err(errors::Error::InvalidCrumbFieldName {
                field_name: crumb.crumb_request_field,
            }.into());
        }
        Ok(crumb)
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    static JENKINS_URL: &'static str = mockito::SERVER_URL;

    #[test]
    fn get_invalid_crumb() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let _mock = mockito::mock("GET", "/crumbIssuer/api/json?depth=1")
            .with_body(
                r#"
{
        "_class":"hudson.security.csrf.DefaultCrumbIssuer",
        "crumb":"1234567890abcdef",
        "crumbRequestField":"Invalid-Crumb"
}
"#,
            )
            .create();

        let crumb = jenkins_client.get_csrf();

        assert!(crumb.is_err());
        assert_eq!(
            format!("{:?}", crumb),
            r#"Err(InvalidCrumbFieldName { field_name: "Invalid-Crumb" })"#
        );
    }

}
