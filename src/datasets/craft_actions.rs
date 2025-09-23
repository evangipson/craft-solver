use crate::{datasets::craft_action::CraftAction, files::from_file::FromFile};
use serde_derive::Deserialize;

/// Represents all crafting actions.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct CraftActions {
    pub craft_actions: Vec<CraftAction>,
}

impl CraftActions {
    /// Gets a crafting action by `id`.
    pub fn get_action_by_id(&self, id: &str) -> Option<CraftAction> {
        self.craft_actions
            .clone()
            .into_iter()
            .find(|ca| ca.id.eq(id))
    }

    /// Gets all crafting actions except ones that `action` any `affixes`.
    pub fn get_actions_except(&self, actions: &[String], affixes: &[String]) -> Vec<CraftAction> {
        self.craft_actions
            .clone()
            .into_iter()
            .filter(|ca| {
                !ca.outcomes
                    .iter()
                    .any(|o| actions.contains(&o.action) && affixes.contains(&o.affix))
            })
            .collect()
    }
}

impl FromFile for CraftActions {}
