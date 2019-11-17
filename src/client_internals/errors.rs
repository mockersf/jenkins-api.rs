use std::fmt;

use thiserror::Error;

/// Wrapper `Result` type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Errors that can be thrown
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid url for {expected}: {url}")]
    ///  Error thrown when a link between objects has an unexpected format
    InvalidUrl {
        /// URL found
        url: String,
        /// Expected URL type
        expected: ExpectedType,
    },

    #[error("invalid crumbfield '{field_name}', expected 'Jenkins-Crumb'")]
    ///  Error thrown when CSRF protection use an unexpected field name
    InvalidCrumbFieldName {
        /// Field name provided by Jenkins api for crumb
        field_name: String,
    },

    #[error("illegal argument: '{message}'")]
    ///  Error thrown when building a parameterized job with an invalid parameter
    IllegalArgument {
        /// Exception message provided by Jenkins
        message: String,
    },

    #[error("illegal state: '{message}'")]
    ///  Error thrown when building a job with invalid parameters
    IllegalState {
        /// Exception message provided by Jenkins
        message: String,
    },

    #[error("can't build a job remotely with parameters")]
    ///  Error when trying to remotely build a job with parameters
    UnsupportedBuildConfiguration,

    #[error("can't do '{action}' on a {object_type} of type {variant_name}")]
    ///  Error when trying to do an action on an object not supporting it
    InvalidObjectType {
        /// Object type
        object_type: ExpectedType,
        /// Variant name
        variant_name: String,
        /// Action
        action: Action,
    },
}

/// Possible type of URL expected in links between items
#[derive(Debug, Copy, Clone)]
pub enum ExpectedType {
    /// a `Build`
    Build,
    /// a `Job`
    Job,
    /// a `QueueItem`
    QueueItem,
    /// a `View`
    View,
    /// a `ShortView`
    ShortView,
    /// a `MavenArtifactRecord`
    MavenArtifactRecord,
}

impl fmt::Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpectedType::Build => write!(f, "Build"),
            ExpectedType::Job => write!(f, "Job"),
            ExpectedType::QueueItem => write!(f, "QueueItem"),
            ExpectedType::View => write!(f, "View"),
            ExpectedType::ShortView => write!(f, "ShortView"),
            ExpectedType::MavenArtifactRecord => write!(f, "MavenArtifactRecord"),
        }
    }
}

/// Possible action done on an object
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Get a field
    GetField(&'static str),
    /// Get linked item
    GetLinkedItem(ExpectedType),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::GetField(field) => write!(f, "get field '{}'", field),
            Action::GetLinkedItem(item) => write!(f, "get linked item '{}'", item),
        }
    }
}
