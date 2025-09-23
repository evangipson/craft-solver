use crate::datasets::affix_tier::AffixTier;

/// An affix candidate for crafting.
#[derive(Clone)]
pub struct AffixCandidate {
    pub affix: String,
    pub affix_tier: AffixTier,
    pub weight: u16,
}
