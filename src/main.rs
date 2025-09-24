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
        "rare",
        82,
        vec![Modifier::from_value(&solver, "max-life", 180)],
        vec![
            Modifier::from_tier(&solver, "chaos-res", 1),
            Modifier::from_tier(&solver, "fire-res", 2),
        ],
    );

    // run the simulation
    solver.simulate(&target_state, 15000, 20);
}
