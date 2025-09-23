use serde_derive::Deserialize;

/// Represents an item class.
#[derive(Default, Deserialize, PartialEq)]
pub struct Class {
    pub name: String,
    pub id: String,
}
