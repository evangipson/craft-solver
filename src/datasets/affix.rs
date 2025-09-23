use serde_derive::Deserialize;

/// Represents a single affix.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct Affix {
    pub name: String,
    pub id: String,
    pub tags: Option<Vec<String>>,
}
