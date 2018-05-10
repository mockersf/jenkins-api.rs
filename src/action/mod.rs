//! Types to parse the actions that triggered a `Build`

use serde::Deserializer;

mod parameters;
pub use self::parameters::Parameter;
mod causes;
pub use self::causes::Cause;
pub mod git;

tagged_enum_or_default!(
    /// An `Action` of a `Build`
    pub enum Action {
        /// An action holding parameters
        ParametersAction (_class = "hudson.model.ParametersAction") {
            /// The list of parameters
            parameters: Vec<Parameter>,
        },
        /// An action listing causes
        CauseAction (_class = "hudson.model.CauseAction") {
            /// The list of causes
            causes: Vec<Cause>,
        },
        /// An action describing a Git change
        GitBuildData (_class = "hudson.plugins.git.util.BuildData" ) {
            /// Name of the SCM
            scm_name: String,
            /// Last revision that was built
            last_built_revision: git::Revision,
            /// URLs to the SCM
            remote_urls: Vec<String>,
            /// Builds and their branches
            builds_by_branch_name: git::BuildsByBranch,
        },
        /// An action for a git tag
        GitTagAction (_class = "hudson.plugins.git.GitTagAction" ) {
        },
        /// An action for a repo tag
        RepoTagAction (_class = "hudson.plugins.repo.TagAction" ) {
        },
        /// An action on time in queue
        TimeInQueueAction (_class = "jenkins.metrics.impl.TimeInQueueAction" ) {
        },
        /// An action from pipelines
        EnvActionImpl (_class = "org.jenkinsci.plugins.workflow.cps.EnvActionImpl" ) {
        },
        /// An action from pipelines
        FlowGraphAction (_class = "org.jenkinsci.plugins.workflow.job.views.FlowGraphAction" ) {
        },
    }
);
