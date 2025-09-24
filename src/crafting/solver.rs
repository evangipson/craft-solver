use crate::{
    crafting::crafter::Crafter,
    datasets::{
        class_tiers::ClassTiers, craft_action::CraftAction, craft_actions::CraftActions,
        items::Items, modifiers::Modifiers,
    },
    files::from_file::FromFile,
    items::item_state::ItemState,
};
use logger::{log_debug, log_info};
use rand::distr::weighted::WeightedIndex;
use rand::{distr::Distribution, seq::IndexedRandom};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

/// Represents a collection of all datasets and crafting functionality.
#[derive(Default)]
pub struct Solver {
    pub modifiers: Modifiers,
    pub items: Items,
    pub craft_actions: CraftActions,
    pub class_tiers: ClassTiers,
    // other datasets would go here, e.g., omens, currencies, etc.
}

impl Solver {
    /// Creates a new [`Solver`].
    pub fn new() -> Self {
        Solver {
            modifiers: Modifiers::default().populate("assets/modifiers.toml"),
            items: Items::default().populate("assets/items.toml"),
            craft_actions: CraftActions::default().populate("assets/craft_actions.toml"),
            class_tiers: ClassTiers::default().populate("assets/class_tiers.toml"),
        }
    }

    /// Simulates attaining the `target_state` over an amount of `runs`, and
    /// reports the results.
    pub fn simulate(&self, target_state: &ItemState, runs: u32, steps_per_run: u32) {
        // Wrap shared state in Arc and Mutex to allow safe, multi-threaded access.
        let best_cost = Arc::new(Mutex::new(f32::MAX));
        let best_sequence = Arc::new(Mutex::new(vec![]));

        let simulation_start = Instant::now();
        log_info!("starting simulation ({runs} runs) to find the best crafting sequence.");

        thread::scope(|s| {
            for i in 0..runs {
                // Clone the Arcs for each new thread. This is a cheap operation.
                let best_cost_clone = Arc::clone(&best_cost);
                let best_sequence_clone = Arc::clone(&best_sequence);

                s.spawn(move || {
                    let run_start = Instant::now();
                    log_debug!("starting run {i}.");

                    let mut crafted_item = ItemState::new(
                        &target_state.base,
                        "normal",
                        target_state.item_level,
                        vec![],
                        vec![],
                    );

                    let mut current_cost = 0.0;
                    let mut sequence: Vec<String> = Vec::new();

                    let mut rng = rand::rng();
                    for _ in 0..steps_per_run {
                        let good_modifiers = crafted_item.get_good_modifiers(target_state);
                        let good_actions = if !good_modifiers.is_empty() {
                            self.craft_actions
                                .get_actions_except(
                                    vec!["remove".to_owned(), "replace".to_owned()].as_slice(),
                                    &good_modifiers
                                        .iter()
                                        .map(|gm| gm.0.to_string())
                                        .collect::<Vec<String>>(),
                                )
                                .into_iter()
                                .filter(|ca| self.is_valid_crafting_action(ca, &crafted_item))
                                .collect::<Vec<CraftAction>>()
                        } else {
                            self.craft_actions
                                .craft_actions
                                .clone()
                                .into_iter()
                                .filter(|ca| self.is_valid_crafting_action(ca, &crafted_item))
                                .collect()
                        };

                        let good_action_ids = good_actions
                            .iter()
                            .map(|ca| ca.id.to_owned())
                            .collect::<Vec<_>>();

                        if good_action_ids.is_empty() {
                            log_debug!("can't find any good crafting actions!");
                            break;
                        }

                        let action_id = good_action_ids.choose(&mut rng).unwrap();
                        self.apply_crafting_action(&mut crafted_item, action_id);
                        current_cost +=
                            self.craft_actions.get_action_by_id(action_id).unwrap().cost;
                        sequence.push(action_id.clone());

                        if crafted_item.meets_target(target_state) {
                            // Lock the mutex to safely access and modify the shared state.
                            let mut locked_cost = best_cost_clone.lock().unwrap();
                            if current_cost < *locked_cost {
                                let mut locked_sequence = best_sequence_clone.lock().unwrap();
                                *locked_cost = current_cost;
                                *locked_sequence = sequence;

                                log_info!(
                                    "found a new best sequence (cost: {:.2}) after {} iterations!",
                                    *locked_cost,
                                    i + 1
                                );
                                crafted_item.display();
                            }
                            break;
                        }

                        // clear out any actions from the crafted item's target
                        self.reset_item_target_action(&mut crafted_item, action_id);
                    }

                    log_debug!(
                        "finished run {} ({:.2}s)",
                        i,
                        run_start.elapsed().as_secs_f32()
                    );
                });
            }
        });

        // lock the shared variables one last time to read the final results
        let final_best_cost = best_cost.lock().unwrap();
        let final_best_sequence = best_sequence.lock().unwrap();

        // the simulation is complete after the mutable members are successfully unlocked
        let elapsed_time = simulation_start.elapsed().as_secs_f32();
        log_info!("simulation complete ({elapsed_time:.2}s).");

        if final_best_sequence.is_empty() {
            log_info!("could not find a successful sequence within the given parameters.");
        } else {
            log_info!("optimal sequence found: {:?}", *final_best_sequence);
            log_info!("total cost: ~{:.2} exalted orbs", *final_best_cost);
        }
    }

    /// Applies a crafting action to an item.
    fn apply_crafting_action(&self, item_state: &mut ItemState, action_id: &str) {
        let action = self
            .craft_actions
            .get_action_by_id(action_id)
            .expect("Unknown crafting action");

        log_debug!("using '{}'...", action.name);

        let mut rng = rand::rng();
        let dist = WeightedIndex::new(
            action
                .outcomes
                .iter()
                .map(|o| o.probability)
                .collect::<Vec<f32>>(),
        )
        .expect("Could not distribute modifier weights.");
        let outcome = &action.outcomes[dist.sample(&mut rng)];
        match outcome.action.as_str() {
            "add" => {
                for _ in 0..outcome.count.unwrap_or(1) {
                    let outcome_affix = self.get_outcome_affix(outcome, item_state);
                    if outcome_affix.is_empty() {
                        log_debug!("couldn't find a good \"add\" outcome!");
                        break;
                    }
                    if self.add_random_affix(
                        &self.items.items,
                        &self.class_tiers.class_tiers,
                        &self.modifiers,
                        item_state,
                        outcome_affix,
                        1,
                    ) {
                        item_state.clear_affix_target(outcome_affix.to_owned());
                    }
                }
            }
            "remove" => {
                for _ in 0..outcome.count.unwrap_or(1) {
                    let outcome_affix = self.get_outcome_affix(outcome, item_state);
                    if outcome_affix.is_empty() {
                        log_debug!("couldn't find a good \"remove\" outcome!");
                        break;
                    }
                    if self.remove_random_affix(item_state, outcome_affix, 1) {
                        item_state.clear_affix_target(outcome_affix.to_owned());
                    }
                }
            }
            "replace" => {
                let target_affix = self.get_outcome_affix(outcome, item_state);
                self.remove_random_affix(item_state, target_affix, 1);
                self.add_random_affix(
                    &self.items.items,
                    &self.class_tiers.class_tiers,
                    &self.modifiers,
                    item_state,
                    target_affix,
                    1,
                );
                item_state.clear_affix_target(target_affix.to_owned());
            }
            "target" => {
                if action.targets_lowest_tier() {
                    item_state.target_lowest_tier();
                }
                if action.targets_affix() {
                    item_state.target_affixes(&outcome.affix);
                }
                item_state.set_next_action(action.currency);
            }
            _ => log_debug!("Unknown action type: {}", outcome.action),
        }

        // Update rarity based on the new number of affixes
        let num_affixes = item_state.prefixes.len() + item_state.suffixes.len();
        if !item_state.rarity.eq("rare") {
            item_state.rarity = if num_affixes == 0 && !item_state.rarity.eq("magic") {
                "normal".to_owned()
            } else if num_affixes >= 3 || action_id.eq("regal") {
                "rare".to_owned()
            } else if num_affixes > 0 && num_affixes < 3 {
                "magic".to_owned()
            } else {
                item_state.rarity.to_owned()
            };
            log_debug!("updated item rarity to {}", item_state.rarity);
        }
    }
}

impl Crafter for Solver {}
