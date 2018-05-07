/// Errors that can be thrown
#[derive(Debug, Fail)]
pub enum Error {
    /// Error thrown when a link between objects has an unexpected format
    #[fail(display = "invalid url for {}: {}", expected, url)]
    InvalidUrl {
        /// URL found
        url: String,
        /// Expected URL type
        expected: String,
    },
    /// Error thrown when CSRF protection use an unexpected field name
    #[fail(display = "invalid crumbfield '{}', expected 'Jenkins-Crumb'", field_name)]
    InvalidCrumbFieldName {
        /// Field name provided by Jenkins api for crumb
        field_name: String,
    },
    /// Error thrown when building a parameterized job with an invalid parameter
    #[fail(display = "illegal argument: '{}'", message)]
    IllegalArgument {
        /// Exception message provided by Jenkins
        message: String,
    },
    /// Error thrown when building a job with invalid parameters
    #[fail(display = "illegal state: '{}'", message)]
    IllegalState {
        /// Exception message provided by Jenkins
        message: String,
    },
}
