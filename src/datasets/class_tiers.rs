use crate::{datasets::class_tier::ClassTier, files::from_file::FromFile};
use serde_derive::Deserialize;

/// Represents all class-based tiered modifiers.
#[derive(Default, Deserialize, PartialEq)]
pub struct ClassTiers {
    pub class_tiers: Vec<ClassTier>,
}

impl FromFile for ClassTiers {}
