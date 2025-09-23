use rand::Rng;
use serde_derive::Deserialize;

/// Represents a list of tiers for a hybrid modifier.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct HybridModifierTier {
    pub affix: String,
    pub range: Option<[u16; 2]>,
    pub min: Option<[u16; 2]>,
    pub max: Option<[u16; 2]>,
}

impl HybridModifierTier {
    /// Gets a hybrid modifier value from [`HybridModifierTier::range`],
    /// or [`HybridModifierTier::min`] and [`HybridModifierTier::max`].
    pub fn get_value(&self) -> u16 {
        let mut rng = rand::rng();
        if let Some(range) = self.range {
            rng.random_range(range[0]..range[1])
        } else {
            let min = self
                .min
                .expect("must have a min defined with max for a hybrid modifier tier!");
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
        }
    }
}
