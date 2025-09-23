use crate::datasets::affix_tier::AffixTier;
use serde_derive::Deserialize;

/// Represents a list of tiered modifiers for an item class.
#[derive(Default, Deserialize, PartialEq)]
pub struct ClassTier {
    pub classes: Vec<String>,
    pub prefixes: Vec<AffixTier>,
    pub suffixes: Vec<AffixTier>,
}

impl ClassTier {
    /// Gets the weight of all prefixes for a modifier tier list.
    pub fn get_prefixes_weight(&self) -> u16 {
        self.prefixes.iter().map(|p| p.get_tier_weight()).sum()
    }

    /// Gets the weight of all suffixes for a modifier tier list.
    pub fn get_suffixes_weight(&self) -> u16 {
        self.suffixes.iter().map(|p| p.get_tier_weight()).sum()
    }

    /// Gets the weight of all affixes for a modifier tier list.
    pub fn get_total_weight(&self) -> u16 {
        self.get_prefixes_weight() + self.get_suffixes_weight()
    }
}
