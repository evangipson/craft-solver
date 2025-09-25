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
        self.prefixes.iter().map(|p| p.get_all_tiers_weight()).sum()
    }

    /// Gets the weight of all suffixes for a modifier tier list.
    pub fn get_suffixes_weight(&self) -> u16 {
        self.suffixes.iter().map(|p| p.get_all_tiers_weight()).sum()
    }

    /// Gets the weight of all affixes for a modifier tier list.
    pub fn get_total_weight(&self) -> u16 {
        self.get_prefixes_weight() + self.get_suffixes_weight()
    }

    /// Gets the weight of all affixes for a modifier tier list.
    pub fn get_total_weight_for_item_level(&self, item_level: u8) -> u32 {
        self.prefixes
            .iter()
            .flat_map(|prefix| {
                prefix
                    .tiers
                    .iter()
                    .filter(|tier| tier.item_level <= item_level)
                    .map(|tier| tier.weight as u32)
                    .collect::<Vec<u32>>()
            })
            .sum::<u32>()
            + self
                .suffixes
                .iter()
                .flat_map(|suffix| {
                    suffix
                        .tiers
                        .iter()
                        .filter(|tier| tier.item_level <= item_level)
                        .map(|tier| tier.weight as u32)
                        .collect::<Vec<u32>>()
                })
                .sum::<u32>()
    }

    /// Gets the weight of an `affix` `tier`, defaults to `0`.
    pub fn get_weight_of_tier(&self, affix: &str, tier: u8) -> u16 {
        if let Some(prefix) = self.prefixes.iter().find(|p| p.affix.eq(affix)) {
            prefix.get_tier_weight(tier).unwrap_or_default()
        } else if let Some(suffix) = self.suffixes.iter().find(|s| s.affix.eq(affix)) {
            suffix.get_tier_weight(tier).unwrap_or_default()
        } else {
            0
        }
    }

    /// Gets the highest possible `affix` tier for `item_level`.
    pub fn get_highest_affix_tier_for_item_level(&self, affix: &str, item_level: u8) -> u8 {
        if let Some(prefix) = self.prefixes.iter().find(|p| p.affix.eq(affix)) {
            prefix.get_highest_tier_for_item_level(item_level)
        } else if let Some(suffix) = self.suffixes.iter().find(|s| s.affix.eq(affix)) {
            suffix.get_highest_tier_for_item_level(item_level)
        } else {
            0
        }
    }
}
