use crate::datasets::modifier_tier::ModifierTier;
use serde_derive::Deserialize;

/// Represents a list of tiers for an affix with ranges of values.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct AffixTier {
    pub affix: String,
    pub tiers: Vec<ModifierTier>,
}

impl AffixTier {
    /// Gets the weight of all tiers for an affix.
    pub fn get_all_tiers_weight(&self) -> u16 {
        self.tiers.iter().map(|t| t.weight).sum()
    }

    /// Gets the highest possible tier for an item level.
    pub fn get_highest_tier_for_item_level(&self, item_level: u8) -> u8 {
        self.tiers
            .iter()
            .map(|t| t.item_level)
            .find(|ilvl| ilvl <= &item_level)
            .unwrap_or_default()
    }

    /// Gets a specific tier's weight for an affix.
    pub fn get_tier_weight(&self, tier: u8) -> Option<u16> {
        if tier >= self.tiers.len() as u8 {
            None
        } else {
            Some(self.tiers[tier as usize].weight)
        }
    }

    /// Gets the minimum value for an affix modifier by `tier`.
    pub fn get_minimum_tier_value(&self, tier: u8) -> Option<u16> {
        if tier >= self.tiers.len() as u8 {
            None
        } else {
            Some(self.tiers[tier as usize].get_minimum_value())
        }
    }

    /// Gets the tier of an affix modifier by `value`.
    pub fn get_value_tier(&self, value: u16) -> Option<u8> {
        self.tiers
            .iter()
            .position(|t| t.get_minimum_value() <= value)
            .map(|tier| tier as u8)
    }
}
