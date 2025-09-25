use crate::{
    datasets::class_tier::ClassTier,
    items::{item_state::ItemState, modifier::Modifier},
};

/// Responsible for all item combination actions.
pub trait Recombiner {
    /// Selects a base from two items of the same class.
    fn select_item(
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
    fn calculate_success_chance(
        &self,
        base_type: &ClassTier,
        item_level: u8,
        affix_id: &str,
        coefficient: u32,
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

        // return the percent chance of the recombination
        ((coefficient * sum_of_weights) as f32 / total_weight as f32) * 100.0
    }
}
