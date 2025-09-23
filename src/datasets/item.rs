use crate::datasets::affix_range::AffixRange;
use crate::datasets::stat::Stat;
use serde_derive::Deserialize;

/// Represents an item.
#[derive(Default, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub class: String,
    pub stats: Option<Vec<Stat>>,
    pub character_level: Option<u8>,
    pub implicits: Option<Vec<AffixRange>>,
}
