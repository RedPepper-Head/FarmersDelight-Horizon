# Native Horizon integration smoke test (source only)

Target: Horizon Nightly 2026-11-08, commit `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`.

The `mixins/farmers_delight_horizon/` directory is an **experimental mixin source bundle** adapted from the Horizon native mod template. It installs a HEAD hook on `Game::break_block`, observed in the target Horizon source, and prints the block position. It does not alter game behavior or register Farmer's Delight content.

**This is not a working mod yet.** It has not been compiled against Horizon or packaged into a valid `.hnmod`. The standalone `cargo test` workflow does not compile this mixin. Do not install the folder or rename it to `.hnmod`.

Before packaging, resolve the exact compatible `horizon-mixins` revision for the target Nightly, verify the injection API against that revision, and generate a valid `horizon-native.json` manifest. Then build using the launcher and check the output logs. Compatibility with Minimap and TreeFall remains untested.

The smoke test is deliberately limited: once it loads, it only proves that a native hook can be injected; it does **not** prove that the cooking library is integrated with the game.
