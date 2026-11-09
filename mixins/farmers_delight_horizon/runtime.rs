//! Injected Horizon runtime module. Kept intentionally independent of the standalone crate.
//! No Farmer's Delight blocks, items, or recipes are registered yet.

pub fn on_block_break(pos: [i32; 3]) {
    eprintln!(
        "[farmers_delight_horizon] Observed block break at {}, {}, {}",
        pos[0], pos[1], pos[2]
    );
}
