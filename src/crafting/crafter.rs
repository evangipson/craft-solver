use crate::{
    crafting::affix_candidate::AffixCandidate,
    datasets::{
        affix_tier::AffixTier, class_tier::ClassTier, craft_outcome::CraftOutcome, item::Item,
        modifiers::Modifiers,
    },
    items::{item_state::ItemState, modifier::Modifier},
};
use logger::log_debug;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};

/// Responsible for all affix-related actions.
pub trait Crafter {
    /// Gets a list of all possible affixes (prefixes or suffixes) that
    /// can roll on an item of a given class and level.
    fn get_possible_affixes(
        &self,
        class_tiers: &[ClassTier],
        item_class: &str,
        item_level: u8,
        affix_type: &str,
    ) -> Vec<AffixCandidate> {
        if let Some(class_tiers) = class_tiers
            .iter()
            .find(|ct| ct.classes.contains(&item_class.to_string()))
        {
            match affix_type {
                "prefix" => &class_tiers.prefixes,
                _ => &class_tiers.suffixes,
            }
            .iter()
            .filter(|affix_tier| {
                affix_tier
                    .tiers
                    .iter()
                    .any(|tier| tier.item_level <= item_level)
            })
            .flat_map(|affix_tier: &AffixTier| {
                affix_tier
                    .tiers
                    .clone()
                    .iter()
                    .map(|tier| AffixCandidate {
                        affix: affix_tier.clone().affix,
                        affix_tier: affix_tier.clone(),
                        weight: tier.weight,
                    })
                    .collect::<Vec<AffixCandidate>>()
            })
            .collect::<Vec<AffixCandidate>>()
        } else {
            vec![]
        }
    }

    /// Selects a random affix based on its weight.
    fn choose_random_affix(
        &self,
        modifiers: Modifiers,
        current_affixes: &[Modifier],
        possible_affixes: &[AffixCandidate],
    ) -> Option<Modifier> {
        if possible_affixes.is_empty() {
            log_debug!("No possible affixes when choosing random modifier.");
            return None;
        }

        let current_affix_ids: Vec<&str> =
            current_affixes.iter().map(|ca| ca.id.as_str()).collect();

        let valid_affixes: Vec<&AffixCandidate> = possible_affixes
            .iter()
            .filter(|t| !current_affix_ids.contains(&t.affix.as_str()))
            .collect();

        let weights: Vec<u64> = valid_affixes
            .clone()
            .into_iter()
            .map(|t| t.weight.into())
            .collect();

        let dist = WeightedIndex::new(weights)
            .expect("Could not make a new weighted index for affix weights.");
        let mut rng = rand::rng();
        let chosen_index = dist.sample(&mut rng);
        let chosen_affix_candidate = &valid_affixes[chosen_index];

        let chosen_modifier_from_db = modifiers
            .get_affix_by_id(&chosen_affix_candidate.affix)
            .unwrap();

        let tier_weights: Vec<u16> = chosen_affix_candidate
            .affix_tier
            .tiers
            .iter()
            .map(|t| t.weight)
            .collect();
        let tier_dist = WeightedIndex::new(tier_weights)
            .expect("Could not make a new weighted index for tier weights.");
        let chosen_tier_index = tier_dist.sample(&mut rng);
        let chosen_tier = &chosen_affix_candidate.affix_tier.tiers[chosen_tier_index];

        let chosen_tier_value = chosen_tier.get_value();

        Some(Modifier {
            // Take ownership of the new String created by .replace()
            name: chosen_modifier_from_db
                .name
                .replace("#", &chosen_tier_value.to_string()),
            // Clone the ID to move an owned String into the struct
            id: chosen_modifier_from_db.id.clone(),
            tier: (chosen_tier_index + 1) as u8,
            value: chosen_tier_value as u16,
        })
    }

    /// Adds a random affix to `item_state`.
    fn add_random_affix(
        &self,
        items: &[Item],
        class_tiers: &[ClassTier],
        modifiers: &Modifiers,
        item_state: &mut ItemState,
        affix_type: &str,
        count: i32,
    ) {
        let current_affixes = [item_state.prefixes.clone(), item_state.suffixes.clone()].concat();

        let affix_list = match affix_type {
            "prefix" => &mut item_state.prefixes,
            "suffix" => &mut item_state.suffixes,
            _ => {
                log_debug!("Error: Invalid affix type");
                return;
            }
        };

        let matched_item = items
            .iter()
            .find(|i| i.name.eq(&item_state.base))
            .expect("could not find matching item!");

        let max_affixes = match affix_type {
            "prefix" => item_state.max_prefixes,
            "suffix" => item_state.max_suffixes,
            _ => 3,
        };

        for _ in 0..count {
            if affix_list.len() as u8 >= max_affixes {
                log_debug!("Cannot add {}. Maximum affixes reached.", affix_type);
                return;
            }

            let possible_affixes = self.get_possible_affixes(
                class_tiers,
                &matched_item.class,
                item_state.item_level,
                affix_type,
            );

            if possible_affixes.is_empty() {
                log_debug!("No possible {} found for this item.", affix_type);
                return;
            }

            if let Some(affix) =
                self.choose_random_affix(modifiers.clone(), &current_affixes, &possible_affixes)
            {
                log_debug!("Added {}: {}", affix_type, affix.name);
                affix_list.push(affix);
            }
        }
    }

    /// Removes a random affix from `item_state`.
    fn remove_random_affix(&self, item_state: &mut ItemState, affix_type: &str, count: i32) {
        let affix_list = match affix_type {
            "prefix" => &mut item_state.prefixes,
            "suffix" => &mut item_state.suffixes,
            _ => {
                log_debug!("Error: Invalid affix type");
                return;
            }
        };

        for _ in 0..count {
            if affix_list.is_empty() {
                log_debug!("No {} to remove.", affix_type);
                return;
            }

            let mut rng = rand::rng();
            let removed_affix = affix_list.swap_remove(rng.random_range(0..affix_list.len()));
            log_debug!("Removed {}: {}", affix_type, removed_affix.name);
        }
    }

    /// Gets an affix ("prefix" or "suffix") from `outcome`.
    fn get_outcome_affix(&self, outcome: &CraftOutcome, item_state: &ItemState) -> &str {
        let is_random_add = outcome.affix == "random" && outcome.action == "add";
        let is_random_remove = outcome.affix == "random" && outcome.action == "remove";

        if (item_state.has_max_affixes() && is_random_add)
            || (item_state.has_no_affixes() && is_random_remove)
        {
            ""
        } else if (item_state.has_max_prefixes() && is_random_add)
            || (item_state.has_no_prefixes() && is_random_remove)
        {
            "suffix"
        } else if (item_state.has_max_suffixes() && is_random_add)
            || (item_state.has_no_suffixes() && is_random_remove)
        {
            "prefix"
        } else {
            let mut rng = rand::rng();
            if (outcome.affix == "random" && rng.random_bool(0.5)) || outcome.affix == "prefix" {
                "prefix"
            } else {
                "suffix"
            }
        }
    }

    /// Gets a minimum `affix` value from `class_tiers` by `tier`.
    fn get_minimum_affix_value(&self, class_tiers: &[ClassTier], affix: String, tier: u8) -> u16 {
        class_tiers
            .iter()
            .flat_map(|ct| [ct.prefixes.as_slice(), ct.suffixes.as_slice()].concat())
            .find(|a| a.affix.eq(&affix))
            .map(|a| a.get_minimum_tier_value(tier).unwrap_or_default())
            .unwrap_or_default()
    }

    /// Gets the `affix` tier from `class_tiers` by `value`.
    fn get_affix_tier(&self, class_tiers: &[ClassTier], affix: String, value: u16) -> u8 {
        class_tiers
            .iter()
            .flat_map(|ct| [ct.prefixes.as_slice(), ct.suffixes.as_slice()].concat())
            .find(|a| a.affix.eq(&affix))
            .map(|a| a.get_value_tier(value).unwrap_or_default())
            .unwrap_or_default()
    }
}
