use serde_derive::Deserialize;

/// Represents an affix with ranges of values.
#[derive(Default, Deserialize, PartialEq)]
pub struct AffixRange {
    pub affix: String,
    pub range: Option<[u8; 2]>,
    pub min: Option<[u8; 2]>,
    pub max: Option<[u8; 2]>,
}
