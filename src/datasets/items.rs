use crate::{
    datasets::{class::Class, item::Item},
    files::from_file::FromFile,
};
use serde_derive::Deserialize;

/// Represents a collection of items and their related item class.
#[derive(Default, Deserialize, PartialEq)]
pub struct Items {
    pub classes: Vec<Class>,
    pub items: Vec<Item>,
}

impl FromFile for Items {}
