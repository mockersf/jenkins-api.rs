//! Types to parse the parameters of a `Build`

use serde::Deserializer;

tagged_enum_or_default!(
    /// A `Parameter` on a `ParametersAction`
    pub enum Parameter {
        /// A boolean parameter
        BooleanParameterValue (_class = "hudson.model.BooleanParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: bool,
        },
        /// A file parameter
        FileParameterValue (_class = "hudson.model.FileParameterValue") {
            /// The parameter name
            name: String,
        },
        /// A password parameter
        PasswordParameterValue (_class = "hudson.model.PasswordParameterValue") {
            /// The parameter name
            name: String,
        },
        /// A run parameter
        RunParameterValue (_class = "hudson.model.RunParameterValue") {
            /// The parameter name
            name: String,
            /// Name of the `Job` for this parameter
            job_name: String,
            /// Number of the `Build` passed
            number: String,
        },
        /// A string parameter
        StringParameterValue (_class = "hudson.model.StringParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: String,
        },
        /// A text parameter
        TextParameterValue (_class = "hudson.model.TextParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: String,
        },
    }
);
