use crate::datasets::craft_outcome::CraftOutcome;
use serde_derive::Deserialize;

/// Represents a crafting action.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct CraftAction {
    pub name: String,
    pub id: String,
    pub cost: f32,
    pub currency: Option<String>,
    pub rarity: Option<String>,
    pub outcomes: Vec<CraftOutcome>,
}

impl CraftAction {
    pub fn only_adds_prefix(&self) -> bool {
        self.does_only_affix_action("prefix", "add")
    }

    pub fn only_adds_suffix(&self) -> bool {
        self.does_only_affix_action("suffix", "add")
    }

    pub fn only_removes_prefix(&self) -> bool {
        self.does_only_affix_action("prefix", "remove")
    }

    pub fn only_removes_suffix(&self) -> bool {
        self.does_only_affix_action("suffix", "remove")
    }

    pub fn only_targets_prefix(&self) -> bool {
        self.does_only_affix_action("prefix", "target")
    }

    pub fn only_targets_suffix(&self) -> bool {
        self.does_only_affix_action("suffix", "target")
    }

    pub fn adds_prefix(&self) -> bool {
        self.does_affix_action("prefix", "add")
    }

    pub fn adds_suffix(&self) -> bool {
        self.does_affix_action("suffix", "add")
    }

    pub fn removes_prefix(&self) -> bool {
        self.does_affix_action("prefix", "remove")
    }

    pub fn removes_suffix(&self) -> bool {
        self.does_affix_action("suffix", "remove")
    }

    pub fn targets_prefix(&self) -> bool {
        self.does_affix_action("prefix", "target")
    }

    pub fn targets_suffix(&self) -> bool {
        self.does_affix_action("suffix", "target")
    }

    pub fn targets_lowest_tier(&self) -> bool {
        self.does_only_affix_action("lowest", "target")
    }

    pub fn targets_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("target"))
    }

    pub fn adds_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("add"))
    }

    pub fn removes_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("remove"))
    }

    pub fn replaces_affix(&self) -> bool {
        self.outcomes.iter().all(|o| o.action.eq("replace"))
    }

    pub fn expects_chaos(&self) -> bool {
        self.targets_affix() && self.currency.clone().unwrap_or_default().eq("chaos")
    }

    pub fn expects_annul(&self) -> bool {
        self.targets_affix() && self.currency.clone().unwrap_or_default().eq("annul")
    }

    fn does_affix_action(&self, affix: &str, action: &str) -> bool {
        self.outcomes
            .iter()
            .any(|o| o.action.eq(action) && o.affix.eq(affix))
    }

    fn does_only_affix_action(&self, affix: &str, action: &str) -> bool {
        self.outcomes
            .iter()
            .all(|o| o.action.eq(action) && o.affix.eq(affix))
    }
}
