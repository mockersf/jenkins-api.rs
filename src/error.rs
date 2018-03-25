/// Errors that can be thrown
#[derive(Debug, Fail)]
pub enum Error {
    /// Error thrown when a link between objects has an unexpected format
    #[fail(display = "invalid url for {}: {}", expected, url)]
    InvalidUrl { url: String, expected: String },
    /// Error thrown when CSRF protection use an unexpected field name
    #[fail(display = "invalid crumbfield '{}', expected 'Jenkins-Crumb'", field_name)]
    InvalidCrumbFieldName { field_name: String },
}
