//! types to parse the actions that triggered a `Build`

use serde::Deserializer;

tagged_enum_or_default!(
    /// A `Parameter` on a `ParametersAction`
    pub enum Parameter {
        /// A boolean parameter
        BooleanParameterValue ("hudson.model.BooleanParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: bool,
        },
        /// A file parameter
        FileParameterValue ("hudson.model.FileParameterValue") {
            /// The parameter name
            name: String,
        },
        /// A password parameter
        PasswordParameterValue ("hudson.model.PasswordParameterValue") {
            /// The parameter name
            name: String,
        },
        /// A run parameter
        RunParameterValue ("hudson.model.RunParameterValue") {
            /// The parameter name
            name: String,
            /// Name of the `Job` for this parameter
            job_name: String,
            /// Number of the `Build` passed
            number: String,
        },
        /// A string parameter
        StringParameterValue ("hudson.model.StringParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: String,
        },
        /// A text parameter
        TextParameterValue ("hudson.model.TextParameterValue") {
            /// The parameter name
            name: String,
            /// The parameter value
            value: String,
        },
    }
);

tagged_enum_or_default!(
    /// A `Cause` on a `CauseAction`
    pub enum Cause {
        /// Caused by a user
        UserIdCause ("hudson.model.Cause$UserIdCause") {
            /// Short description of the cause
            short_description: String,
            /// User ID responsible
            user_id: String,
            /// User name responsible
            user_name: String,
        },
        /// Caused remotely
        RemoteCause ("hudson.model.Cause$RemoteCause") {
            /// Short description of the cause
            short_description: String,
            /// addr that triggered
            addr: String,
            /// note provided when triggering the build
            note: Option<String>,
        },
        /// Caused by another project
        UpstreamCause ("hudson.model.Cause$UpstreamCause") {
            /// Short description of the cause
            short_description: String,
            /// `Build` number that triggered this `Build`
            upstream_build: u32,
            /// `Job` whose `Build` triggered this `Build`
            upstream_project: String,
            /// URL to the upstream `Build`
            upstream_url: String,
        },
        /// Caused by a timer
        TimerTriggerCause ("hudson.triggers.TimerTrigger$TimerTriggerCause") {
            /// Short description of the cause
            short_description: String,
        },
        /// Caused by a SCM change
        SCMTriggerCause ("hudson.triggers.SCMTrigger$SCMTriggerCause") {
            /// Short description of the cause
            short_description: String,
        },
    }
);

tagged_enum_or_default!(
    /// An `Action` of a `Build`
    pub enum Action {
        /// An action holding parameters
        ParametersAction ("hudson.model.ParametersAction") {
            /// The list of parameters
            parameters: Vec<Parameter>,
        },
        /// An action listing causes
        CauseAction ("hudson.model.CauseAction") {
            /// The list of causes
            causes: Vec<Cause>,
        },
    }
);
