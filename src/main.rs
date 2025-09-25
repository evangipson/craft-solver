use craft_solver::{
    crafting::solver::Solver,
    items::{item_state::ItemState, modifier::Modifier},
};
use logger::log_severity::LogSeverity;

fn main() {
    // show any important logs by log severity level
    logger::set_logging_severity(LogSeverity::Info);

    // set up the solver, which runs the crafting actions
    let solver = Solver::new();

    // define the end state
    let target_state = ItemState::new(
        "Warlord Cuirass",
        "chest",
        "rare",
        82,
        vec![Modifier::from_value(&solver, "max-life", 180)],
        vec![
            Modifier::from_tier(&solver, "chaos-res", 1),
            Modifier::from_tier(&solver, "fire-res", 2),
        ],
    );

    // =======================
    // TEST RECOMBINATOR STUFF
    // =======================
    let right_item = ItemState::new(
        "Warlord Cuirass",
        "chest",
        "rare",
        82,
        vec![Modifier::from_value(&solver, "armor", 100)],
        vec![Modifier::from_tier(&solver, "cold-res", 2)],
    );

    let left_mods = vec![
        Modifier::from_value(&solver, "max-life", 180),
        Modifier::from_tier(&solver, "chaos-res", 1),
    ];
    let right_mods = vec![
        Modifier::from_value(&solver, "armor", 100),
        Modifier::from_tier(&solver, "cold-res", 2),
    ];

    solver.recombine(&target_state, &right_item, left_mods, right_mods);
    // ===========================
    // END TEST RECOMBINATOR STUFF
    // ===========================

    // run the simulation
    solver.simulate(&target_state, 100, 20);
}
