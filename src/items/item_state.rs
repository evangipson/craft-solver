use std::collections::HashMap;

use crate::items::modifier::Modifier;
use logger::log_info;

/// Represents the state of an item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemState {
    pub base: String,
    pub rarity: String,
    pub max_prefixes: u8,
    pub max_suffixes: u8,
    pub item_level: u8,
    pub prefixes: Vec<Modifier>,
    pub suffixes: Vec<Modifier>,
    // TODO: add a field for active Omens
}

impl ItemState {
    /// Makes a new [`ItemState`].
    pub fn new(
        base: &str,
        rarity: &str,
        item_level: u8,
        prefixes: Vec<Modifier>,
        suffixes: Vec<Modifier>,
    ) -> Self {
        Self {
            base: base.to_owned(),
            rarity: rarity.to_owned(),
            max_prefixes: 3,
            max_suffixes: 3,
            item_level,
            prefixes,
            suffixes,
        }
    }

    /// Prints a user-friendly representation of an item state.
    pub fn display(&self) {
        let base_rarity_length = self.base.len() + self.rarity.len();
        log_info!("--- {} ({}) ---", self.base, self.rarity);
        log_info!("Item Level: {}", self.item_level);
        if !self.prefixes.is_empty() {
            log_info!("Prefixes:");
            for affix in &self.prefixes {
                log_info!("  - {} ({})", affix.name, affix.value);
            }
        }
        if !self.suffixes.is_empty() {
            log_info!("Suffixes:");
            for affix in &self.suffixes {
                log_info!("  - {} ({})", affix.name, affix.value);
            }
        }
        if self.prefixes.is_empty() && self.suffixes.is_empty() {
            log_info!("  (No affixes)");
        }
        log_info!("{:-^width$}", "", width = base_rarity_length + 8);
    }

    // Checks if an item state meets the target state.
    pub fn meets_target(&self, target: &ItemState) -> bool {
        let has_all_prefixes = target.prefixes.iter().all(|target_affix| {
            self.prefixes
                .iter()
                .any(|prefix| self.meets_modifier(prefix, target_affix))
        });
        let has_all_suffixes = target.suffixes.iter().all(|target_affix| {
            self.suffixes
                .iter()
                .any(|suffix| self.meets_modifier(suffix, target_affix))
        });

        has_all_prefixes && has_all_suffixes
    }

    /// Predicate that determines if `a` is at least `b`.
    pub fn meets_modifier(&self, a: &Modifier, b: &Modifier) -> bool {
        a.id == b.id && a.value >= b.value && a.tier >= b.tier
    }

    /// Gets all "good" modifiers that meet the `target` item state.
    pub fn get_good_modifiers(&self, target: &ItemState) -> HashMap<String, Vec<Modifier>> {
        let good_prefixes = Vec::from_iter(self.prefixes.clone().into_iter().filter(|prefix| {
            target
                .prefixes
                .clone()
                .into_iter()
                .any(|target_affix| self.meets_modifier(prefix, &target_affix))
        }));
        let good_suffixes = Vec::from_iter(self.suffixes.clone().into_iter().filter(|suffix| {
            target
                .suffixes
                .clone()
                .into_iter()
                .any(|target_affix| self.meets_modifier(suffix, &target_affix))
        }));
        HashMap::from([
            if !good_prefixes.is_empty() {
                ("prefix".to_owned(), good_prefixes)
            } else {
                ("".to_owned(), Vec::<Modifier>::new())
            },
            if !good_suffixes.is_empty() {
                ("suffix".to_owned(), good_suffixes)
            } else {
                ("".to_owned(), Vec::<Modifier>::new())
            },
        ])
    }

    pub fn has_max_prefixes(&self) -> bool {
        self.prefixes.len() >= self.max_prefixes.into()
    }

    pub fn has_no_prefixes(&self) -> bool {
        self.prefixes.is_empty()
    }

    pub fn has_max_suffixes(&self) -> bool {
        self.suffixes.len() >= self.max_suffixes.into()
    }

    pub fn has_no_suffixes(&self) -> bool {
        self.suffixes.is_empty()
    }

    pub fn has_no_affixes(&self) -> bool {
        self.has_no_prefixes() && self.has_no_suffixes()
    }

    pub fn has_max_affixes(&self) -> bool {
        self.has_max_prefixes() && self.has_max_suffixes()
    }
}
