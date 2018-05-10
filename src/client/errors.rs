use std::fmt;

/// Errors that can be thrown
#[derive(Debug, Fail)]
pub enum Error {
    /// Error thrown when a link between objects has an unexpected format
    #[fail(display = "invalid url for {}: {}", expected, url)]
    InvalidUrl {
        /// URL found
        url: String,
        /// Expected URL type
        expected: ExpectedType,
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

    /// Error when trying to remotely build a job with parameters
    #[fail(display = "can't build a job remotely with parameters")]
    UnsupportedBuildConfiguration,

    /// Error when trying to act on an object of Unknown variant
    #[fail(display = "can't use an Unknown object as a {}", object_type)]
    UnknownType {
        /// Object type of the Unknown variant
        object_type: ExpectedType,
    },
}

/// Possible type of URL expected in links between items
#[derive(Debug, Copy, Clone)]
pub enum ExpectedType {
    /// URL to a `Build`
    Build,
    /// URL to a `Job`
    Job,
    /// URL to a `QueueItem`
    QueueItem,
    /// URL to a `View`
    View,
}

impl fmt::Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExpectedType::Build => write!(f, "Build"),
            &ExpectedType::Job => write!(f, "Job"),
            &ExpectedType::QueueItem => write!(f, "QueueItem"),
            &ExpectedType::View => write!(f, "View"),
        }
    }
}
