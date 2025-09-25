use crate::{
    crafting::{crafter::Crafter, recombiner::Recombiner},
    datasets::{
        class_tier::ClassTier, class_tiers::ClassTiers, craft_action::CraftAction,
        craft_actions::CraftActions, craft_outcome::CraftOutcome, items::Items,
        modifiers::Modifiers,
    },
    files::from_file::FromFile,
    items::{item_state::ItemState, modifier::Modifier},
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
        // wrap shared state in Arc and Mutex to allow safe, multi-threaded access
        let best_cost = Arc::new(Mutex::new(f32::MAX));
        let best_sequence = Arc::new(Mutex::new(vec![]));

        let simulation_start = Instant::now();
        log_info!("starting simulation ({runs} runs) to find the best crafting sequence.");

        // create a thread scope to spawn a thread for each simulation
        thread::scope(|s| {
            for i in 0..runs {
                // clone the Arcs for each new thread; this is a cheap operation
                let best_cost_clone = Arc::clone(&best_cost);
                let best_sequence_clone = Arc::clone(&best_sequence);

                // spawn a thread in main thread scope for each run
                s.spawn(move || {
                    let run_start = Instant::now();
                    log_debug!("starting run {i}.");

                    let mut crafted_item = ItemState::new(
                        &target_state.base,
                        &target_state.class,
                        "normal",
                        target_state.item_level,
                        vec![],
                        vec![],
                    );

                    let mut current_cost = 0.0;
                    let mut sequence: Vec<String> = Vec::new();

                    let mut rng = rand::rng();

                    // apply `steps_per_run` amount of crafts for each run
                    for _ in 0..steps_per_run {
                        let good_action_ids =
                            self.get_crafting_actions(target_state, &crafted_item);

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
                            // lock the mutex to safely access and modify the shared state.
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
                        i + 1,
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

    pub fn recombine(
        &self,
        left_item: &ItemState,
        right_item: &ItemState,
        left_mods: Vec<Modifier>,
        right_mods: Vec<Modifier>,
    ) -> ItemState {
        let base_item = self.select_item(left_item, right_item, &left_mods, &right_mods);
        let all_mods = [left_mods.as_slice(), right_mods.as_slice()].concat();
        let class_tiers = self.get_class_tiers_for_item(&base_item).expect(
            "cannot find class tiers for the selected item, check the class_tiers.toml file!",
        );

        all_mods.iter().for_each(|m| {
            log_info!(
                "while recombining... success chance for tier {} {} is {:.2}%",
                m.tier,
                m.id,
                self.calculate_success_chance(class_tiers, base_item.item_level, &m.id, 1, m.tier)
            );
        });

        base_item
    }

    /// Applies a crafting action to an item.
    fn apply_crafting_action(&self, item_state: &mut ItemState, action_id: &str) {
        // get the crafting action by id
        let action = self
            .craft_actions
            .get_action_by_id(action_id)
            .expect("Unknown crafting action");
        log_debug!("using '{}'...", action.name);

        // get an outcome from the crafting action
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

        // apply the outcome to the item
        self.apply_outcome_to_item(&action, outcome, item_state);

        // Update rarity based on the new number of affixes
        self.update_item_rarity(item_state, action_id);
    }

    /// Applies a [`CraftOutcome`] to an [`ItemState`].
    fn apply_outcome_to_item(
        &self,
        action: &CraftAction,
        outcome: &CraftOutcome,
        item_state: &mut ItemState,
    ) {
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
                item_state.set_next_action(action.currency.clone());
            }
            _ => log_debug!("Unknown action type: {}", outcome.action),
        }
    }

    /// Updates [`ItemState::rarity`].
    fn update_item_rarity(&self, item_state: &mut ItemState, action_id: &str) {
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

    /// Gets a collection of "good" crafting action ids based on `crafted_item`
    /// and `target_state`.
    fn get_crafting_actions(
        &self,
        target_state: &ItemState,
        crafted_item: &ItemState,
    ) -> Vec<String> {
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
                .filter(|ca| self.is_valid_crafting_action(ca, crafted_item))
                .collect::<Vec<CraftAction>>()
        } else {
            self.craft_actions
                .craft_actions
                .clone()
                .into_iter()
                .filter(|ca| self.is_valid_crafting_action(ca, crafted_item))
                .collect()
        };

        good_actions
            .iter()
            .map(|ca| ca.id.to_owned())
            .collect::<Vec<_>>()
    }

    /// Gets all tiers of modifiers for an `item_state`, using it's base class.
    fn get_class_tiers_for_item(&self, item_state: &ItemState) -> Option<&ClassTier> {
        self.class_tiers
            .class_tiers
            .iter()
            .find(|ct| ct.classes.contains(&item_state.class))
    }
}

impl Crafter for Solver {}

impl Recombiner for Solver {}
