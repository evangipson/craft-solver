use crate::crafting::{crafter::Crafter, solver::Solver};

/// Represents a single modifier on an item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifier {
    pub name: String,
    pub id: String,
    pub tier: u8,
    pub value: u16,
}

impl Modifier {
    /// Makes a new [`Modifier`] from `value`.
    pub fn from_value(solver: &Solver, id: &'static str, value: u16) -> Self {
        Self {
            name: String::new(),
            id: id.to_owned(),
            tier: solver.get_affix_tier(&solver.class_tiers.class_tiers, id.to_string(), value),
            value,
        }
    }

    /// Makes a new [`Modifier`] from `tier`.
    pub fn from_tier(solver: &Solver, id: &'static str, tier: u8) -> Self {
        Self {
            name: String::new(),
            id: id.to_owned(),
            tier,
            value: solver.get_minimum_affix_value(
                &solver.class_tiers.class_tiers,
                id.to_string(),
                tier,
            ),
        }
    }
}
