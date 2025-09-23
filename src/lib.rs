/// Contains all crafting-related behaviors
pub mod crafting {
    pub mod affix_candidate;
    pub mod crafter;
    pub mod solver;
}

/// Contains data entities populated by file
pub mod datasets {
    pub mod affix;
    pub mod affix_range;
    pub mod affix_tier;
    pub mod class;
    pub mod class_tier;
    pub mod class_tiers;
    pub mod craft_action;
    pub mod craft_actions;
    pub mod craft_outcome;
    pub mod hybrid_modifier_tier;
    pub mod item;
    pub mod items;
    pub mod modifier_tier;
    pub mod modifiers;
    pub mod stat;
}

/// Contains all file-related behaviors
pub mod files {
    pub mod from_file;
}

/// Contains all item-related entities and behaviors
pub mod items {
    pub mod item_state;
    pub mod modifier;
}
