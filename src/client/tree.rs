use serde::{Serialize, Serializer};

/// Jenkins tree query parameter
#[derive(Debug)]
pub struct TreeQueryParam {
    /// Name of the key at the root of this tree
    pub keyname: String,
    /// Children keys
    pub subkeys: Vec<TreeQueryParam>,
}
impl Serialize for TreeQueryParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl ToString for TreeQueryParam {
    fn to_string(&self) -> String {
        match self.subkeys.len() {
            0 => self.keyname.clone(),
            _ => format!(
                "{}[{}]",
                self.keyname,
                self.subkeys
                    .iter()
                    .map(TreeQueryParam::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

/// Helper to build a `TreeQueryParam`
///
/// ```
/// jenkins_api::client::TreeBuilder::object("builds")
///     .with_subfield("url")
///     .with_subfield("result")
///     .with_subfield(
///         jenkins_api::client::TreeBuilder::object("actions").with_subfield("causes"),
///     )
///     .build()
/// ```
#[derive(Debug)]
pub struct TreeBuilder {
    tree: TreeQueryParam,
}
impl TreeBuilder {
    /// Create a parent `TreeQueryParam`
    pub fn object(name: &str) -> Self {
        TreeBuilder {
            tree: TreeQueryParam {
                keyname: name.to_string(),
                subkeys: vec![],
            },
        }
    }
    /// Add a field to the `TreeQueryParam`
    pub fn with_subfield<T: Into<TreeQueryParam>>(mut self, subfield: T) -> Self {
        self.tree.subkeys.push(subfield.into());
        self
    }
    /// Build the `TreeQueryParam`
    pub fn build(self) -> TreeQueryParam {
        self.tree
    }
}
impl Into<TreeQueryParam> for TreeBuilder {
    fn into(self) -> TreeQueryParam {
        self.build()
    }
}
impl<'a> Into<TreeQueryParam> for &'a str {
    fn into(self) -> TreeQueryParam {
        TreeQueryParam {
            keyname: self.to_string(),
            subkeys: vec![],
        }
    }
}
impl Into<Option<super::AdvancedQuery>> for TreeQueryParam {
    fn into(self) -> Option<super::AdvancedQuery> {
        Some(super::AdvancedQuery::Tree(self))
    }
}
