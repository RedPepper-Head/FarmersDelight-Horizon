// First native integration smoke test: observe, never cancel or change a block break.
fn observe_block_break(pos: [i32; 3]) {
    crate::game::farmers_delight_horizon::on_block_break(pos);
}
