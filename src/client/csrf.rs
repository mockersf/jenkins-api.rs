use std::fmt;
use hyper;
use failure;

use reqwest::header::{Formatter, Header, Raw};
use reqwest::RequestBuilder;

use error;

use super::{Jenkins, Path};

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
        Ok(())
    }
}
