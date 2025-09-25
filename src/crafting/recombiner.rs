use rand::Rng;

use crate::{
    datasets::class_tier::ClassTier,
    items::{item_state::ItemState, modifier::Modifier},
    random::randomizer,
};

/// Responsible for all item combination actions.
pub trait Recombiner {
    /// Gets the amount of recombine modifiers based on `modifier_count`.
    fn get_amount_of_modifers(&self, modifier_count: u8) -> u8 {
        match modifier_count {
            1 => randomizer::if_more_than(41.0, 1, 0),
            2 => randomizer::if_more_than(66.7, 2, 1),
            3 => randomizer::if_more_than(90.0, 3, randomizer::if_more_than(50.0, 2, 1)),
            4 => randomizer::if_more_than(70.0, 3, randomizer::if_more_than(60.0, 2, 1)),
            5 => randomizer::if_more_than(43.0, 3, 2),
            6 => randomizer::if_more_than(30.0, 3, 2),
            _ => 0,
        }
    }

    fn pick_from_selected_modifiers(
        &self,
        modifier_count: u8,
        prefixes: &[Modifier],
        suffixes: &[Modifier],
    ) -> Vec<Modifier> {
        let affix = if rand::rng().random_bool(0.5) {
            "prefix"
        } else {
            "suffix"
        };

        let mut prefixes_picked: i16 = -1;
        let mut suffixes_picked: i16 = -1;
        (0..modifier_count)
            .map(|_| {
                if (affix.eq("prefix") && prefixes_picked < (prefixes.len() - 1) as i16)
                    || (affix.eq("suffix") && suffixes_picked >= (suffixes.len() - 1) as i16)
                {
                    prefixes_picked += 1;
                    prefixes[prefixes_picked as usize].clone()
                } else {
                    suffixes_picked += 1;
                    suffixes[suffixes_picked as usize].clone()
                }
            })
            .collect::<Vec<Modifier>>()
    }

    /// Selects a base from two items of the same class.
    fn select_recombine_item(
        &self,
        left_item: &ItemState,
        right_item: &ItemState,
        left_mods: &[Modifier],
        right_mods: &[Modifier],
    ) -> ItemState {
        let mut left_mod_weights = left_mods.iter().map(|m| m.weight).collect::<Vec<u16>>();
        let mut right_mod_weights = right_mods.iter().map(|m| m.weight).collect::<Vec<u16>>();

        left_mod_weights.sort();
        right_mod_weights.sort();

        let lowest_left_mod_weight = left_mod_weights.first().unwrap();
        let lowest_right_mod_weight = right_mod_weights.first().unwrap();

        if lowest_left_mod_weight >= lowest_right_mod_weight {
            left_item.clone()
        } else {
            right_item.clone()
        }
    }

    /// Calculates a "success chance" of combining items.
    fn get_modifier_recombine_chance(
        &self,
        base_type: &ClassTier,
        item_level: u8,
        affix_id: &str,
        target_tier: u8,
    ) -> f32 {
        // find the total weight of all mods for the item level
        let total_weight = base_type.get_total_weight_for_item_level(item_level);
        if total_weight == 0 {
            return 0.0;
        }

        // find the highest tier for the affix given the item level
        let highest_tier = base_type.get_highest_affix_tier_for_item_level(affix_id, item_level);
        if target_tier >= highest_tier {
            return 0.0;
        }

        // get the sum of all weights from target tier to highest tier
        let sum_of_weights: u32 = (target_tier..highest_tier)
            .map(|tier| base_type.get_weight_of_tier(affix_id, tier) as u32)
            .sum();

        // chests get a 5x coefficient, spears get an 8x, everything else does not
        let coefficient = if base_type.classes.contains(&"chest".to_owned()) {
            5
        } else if base_type.classes.contains(&"spear".to_owned()) {
            8
        } else {
            1
        };

        // return the percent chance of the recombination
        (coefficient * sum_of_weights) as f32 / total_weight as f32
    }
}
