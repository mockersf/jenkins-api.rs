#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid url for {}: {}", expected, url)]
    InvalidUrl { url: String, expected: String },
    #[fail(display = "invalid crumbfield '{}', expected 'Jenkins-Crumb'", field)]
    InvalidCrumbField { field: String },
}
