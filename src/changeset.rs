//! Types describing changes between two builds

use crate::user::ShortUser;

use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::helpers::Class;

/// Trait implemented by specialization of changesetlist
pub trait ChangeSetList {}

macro_rules! changesetlist_with_common_fields_and_impl {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $field_type:ty,
            )*
            $(private_fields {
                $(
                    $(#[$private_field_attr:meta])*
                    $private_field:ident: $private_field_type:ty
                ),* $(,)*
            })*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            /// Origin of the changes
            pub kind: Option<String>,
            /// Changes in this list
            pub items: Vec<CommonChangeSet>,
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
            $($(
                $(#[$private_field_attr])*
                $private_field: $private_field_type,
            )*)*
        }
        impl ChangeSetList for $name {}
    };
}

changesetlist_with_common_fields_and_impl!(/// A Jenkins `ChangeSetList`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonChangeSetList {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    private_fields {
        #[serde(flatten)]
        other_fields: serde_json::Value,
    }
});
specialize!(CommonChangeSetList => ChangeSetList);

changesetlist_with_common_fields_and_impl!(
    /// No changes recorded
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct EmptyChangeSet {}
);
register_class!("hudson.scm.EmptyChangeLogSet" => EmptyChangeSet);

changesetlist_with_common_fields_and_impl!(
    /// Changes found from git
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GitChangeSetList {}
);
register_class!("hudson.plugins.git.GitChangeSetList" => GitChangeSetList);

changesetlist_with_common_fields_and_impl!(
    /// Changes found from a repo
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct RepoChangeLogSet {}
);
register_class!("hudson.plugins.repo.RepoChangeLogSet" => RepoChangeLogSet);

changesetlist_with_common_fields_and_impl!(
    /// Changes filtered by maven module
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct FilteredChangeLogSet {}
);
register_class!("hudson.maven.FilteredChangeLogSet" => FilteredChangeLogSet);

/// Trait implemented by specialization of changeset
pub trait ChangeSet {}

/// A Change Set
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonChangeSet {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    #[serde(flatten)]
    other_fields: serde_json::Value,
}
specialize!(CommonChangeSet => ChangeSet);
impl ChangeSet for CommonChangeSet {}

/// Changes found from git
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitChangeSet {
    /// Comment
    pub comment: String,
    /// Email of the commit
    pub author_email: String,
    /// ID of the commit
    pub commit_id: String,
    /// Date of the commit
    pub date: String,
    /// Commit message
    pub msg: String,
    /// Timestamp of the commit
    pub timestamp: u64,
    /// ID of the commit
    pub id: String,
    /// Files changed in the commit
    pub affected_paths: Vec<String>,
    /// Author of the commit
    pub author: ShortUser,
    /// Files changed in the commit, and how
    pub paths: Vec<PathChange>,
}
register_class!("hudson.plugins.git.GitChangeSet" => GitChangeSet);
impl ChangeSet for GitChangeSet {}

/// Changes found from a repo
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLogEntry {
    /// ID of the commit
    pub commit_id: Option<String>,
    /// Commit message
    pub msg: String,
    /// Timestamp of the commit
    pub timestamp: i64,
    /// Files changed in the commit
    pub affected_paths: Option<Vec<String>>,
    /// Author of the commit
    pub author: ShortUser,
}
register_class!("hudson.plugins.repo.ChangeLogEntry" => ChangeLogEntry);
impl ChangeSet for ChangeLogEntry {}

/// Edit type on a file
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EditType {
    /// Adding a new file
    Add,
    /// Editing a file
    Edit,
    /// Deleting a file
    Delete,
}

/// A file that was changed
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathChange {
    /// File that was changed
    pub file: String,
    /// How it was changed
    pub edit_type: EditType,
}
