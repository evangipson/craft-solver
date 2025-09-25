use crate::crafting::{crafter::Crafter, solver::Solver};

/// Represents a single modifier on an item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifier {
    pub name: String,
    pub id: String,
    pub tier: u8,
    pub value: u16,
    pub weight: u16,
}

impl Modifier {
    /// Makes a new [`Modifier`] from `value`.
    pub fn from_value(solver: &Solver, id: &'static str, value: u16) -> Self {
        let class_tiers = &solver.class_tiers.class_tiers;
        let tier = solver.get_affix_tier(class_tiers, id.to_string(), value);
        Self {
            name: String::new(),
            id: id.to_owned(),
            tier,
            value,
            weight: solver.get_affix_tier_weight(class_tiers, id.to_string(), tier),
        }
    }

    /// Makes a new [`Modifier`] from `tier`.
    pub fn from_tier(solver: &Solver, id: &'static str, tier: u8) -> Self {
        let class_tiers = &solver.class_tiers.class_tiers;
        Self {
            name: String::new(),
            id: id.to_owned(),
            tier,
            value: solver.get_minimum_affix_value(class_tiers, id.to_string(), tier),
            weight: solver.get_affix_tier_weight(class_tiers, id.to_string(), tier),
        }
    }
}
