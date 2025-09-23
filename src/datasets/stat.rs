use serde_derive::Deserialize;

/// Represents an item's required stats.
#[derive(Default, Deserialize, PartialEq)]
pub struct Stat {
    pub armor: Option<u16>,
    pub dexterity: Option<u16>,
    pub intelligence: Option<u16>,
}
