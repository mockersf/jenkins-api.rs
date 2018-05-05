//! Types to parse the causes of a `Build`

use serde::Deserializer;

tagged_enum_or_default!(
    /// A `Cause` on a `CauseAction`
    pub enum Cause {
        /// Caused by a user
        UserIdCause (_class = "hudson.model.Cause$UserIdCause") {
            /// Short description of the cause
            short_description: String,
            /// User ID responsible
            user_id: String,
            /// User name responsible
            user_name: String,
        },
        /// Caused remotely
        RemoteCause (_class = "hudson.model.Cause$RemoteCause") {
            /// Short description of the cause
            short_description: String,
            /// addr that triggered
            addr: String,
            /// Note provided when triggering the build
            note: Option<String>,
        },
        /// Caused by another project
        UpstreamCause (_class = "hudson.model.Cause$UpstreamCause") {
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
        TimerTriggerCause (_class = "hudson.triggers.TimerTrigger$TimerTriggerCause") {
            /// Short description of the cause
            short_description: String,
        },
        /// Caused by a SCM change
        SCMTriggerCause (_class = "hudson.triggers.SCMTrigger$SCMTriggerCause") {
            /// Short description of the cause
            short_description: String,
        },
    }
);
