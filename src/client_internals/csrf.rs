use failure;

use reqwest::{header::HeaderName, header::HeaderValue, RequestBuilder};

use super::{path::Path, Jenkins};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Crumb {
    crumb: String,
    crumb_request_field: String,
}

impl Jenkins {
    pub(crate) fn add_csrf_to_request(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, failure::Error> {
        if self.csrf_enabled {
            let crumb = self.get_csrf()?;
            debug!("{:?}", crumb);
            Ok(request_builder.header(
                HeaderName::from_lowercase(crumb.crumb_request_field.to_lowercase().as_bytes())?,
                HeaderValue::from_str(&crumb.crumb)?,
            ))
        } else {
            Ok(request_builder)
        }
    }

    pub(crate) fn get_csrf(&self) -> Result<Crumb, failure::Error> {
        let crumb: Crumb = self.get(&Path::CrumbIssuer)?.json()?;
        Ok(crumb)
    }
}
