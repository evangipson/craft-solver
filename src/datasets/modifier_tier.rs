use crate::datasets::hybrid_modifier_tier::HybridModifierTier;
use rand::Rng;
use serde_derive::Deserialize;

/// Represents a list of tiers for a modifier.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct ModifierTier {
    pub range: Option<[u16; 2]>,
    pub min: Option<[u16; 2]>,
    pub max: Option<[u16; 2]>,
    pub value: Option<u16>,
    pub hybrid: Option<Vec<HybridModifierTier>>,
    pub item_level: u8,
    pub weight: u16,
}

impl ModifierTier {
    /// Gets a modifier value, either from [`ModifierTier::range`],
    /// [`ModifierTier::min`] and [`ModifierTier::max`],
    /// [`ModifierTier::hybrid`], or [`ModifierTier::value`].
    pub fn get_value(&self) -> u16 {
        let mut rng = rand::rng();
        if let Some(range) = self.range {
            rng.random_range(range[0]..range[1])
        } else if let Some(min) = self.min {
            let max = self
                .max
                .expect("must have a max defined with min for a modifier tier!");
            let min_val = rng.random_range(min[0]..min[1]);
            let max_val = rng.random_range(max[0]..max[1]);
            if min_val == max_val {
                min_val
            } else if min_val > max_val {
                rng.random_range(max_val..min_val)
            } else {
                rng.random_range(min_val..max_val)
            }
        } else if let Some(hybrid) = &self.hybrid {
            hybrid
                .first()
                .expect("must have at least one hybrid tier defined!")
                .get_value()
        } else {
            self.value.expect("value must be defined!")
        }
    }

    /// Gets the minimum value of a [`ModifierTier`].
    pub fn get_minimum_value(&self) -> u16 {
        if let Some(range) = self.range {
            range[0]
        } else if let Some(min) = self.min {
            min[0]
        } else if let Some(hybrid) = &self.hybrid {
            hybrid
                .first()
                .expect("must have at least one hybrid tier defined!")
                .get_value()
        } else {
            self.value.expect("value must be defined!")
        }
    }
}
