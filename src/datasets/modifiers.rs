use crate::{datasets::affix::Affix, files::from_file::FromFile};
use serde_derive::Deserialize;

/// Represents all modifiers.
#[derive(Default, Deserialize, Clone, PartialEq)]
pub struct Modifiers {
    pub modifiers: Vec<Affix>,
}

impl Modifiers {
    /// Gets an affix by `id`.
    pub fn get_affix_by_id(self, id: &str) -> Option<Affix> {
        self.modifiers.into_iter().find(|m| m.id.eq(id))
    }
}

impl FromFile for Modifiers {}
