use serde_derive::Deserialize;

/// Represents a potential crafting outcome.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct CraftOutcome {
    pub action: String,
    pub affix: String,
    pub count: Option<u8>,
    pub probability: f32,
}
