use crate::datasets::craft_outcome::CraftOutcome;
use serde_derive::Deserialize;

/// Represents a crafting action.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct CraftAction {
    pub name: String,
    pub id: String,
    pub cost: f32,
    pub rarity: Option<String>,
    pub outcomes: Vec<CraftOutcome>,
}

impl CraftAction {
    pub fn adds_prefix(&self) -> bool {
        self.does_affix_action("prefix", "add")
    }

    pub fn removes_prefix(&self) -> bool {
        self.does_affix_action("prefix", "remove")
    }

    pub fn adds_suffix(&self) -> bool {
        self.does_affix_action("suffix", "add")
    }

    pub fn removes_suffix(&self) -> bool {
        self.does_affix_action("suffix", "remove")
    }

    pub fn adds_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("add"))
    }

    pub fn removes_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("remove"))
    }

    fn does_affix_action(&self, affix: &str, action: &str) -> bool {
        self.outcomes
            .iter()
            .any(|o| o.action.eq(action) && o.affix.eq(affix))
    }
}
