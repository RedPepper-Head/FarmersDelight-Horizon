# Build status

Target Horizon Nightly (2026-11-08): `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`

The repository now contains a standalone Rust library with pure cooking-recipe ingredient validation and unit tests. It does **not** contain a Horizon-compatible native mixin bundle or a packaged `.hnmod`.

## Local smoke test

Install Rust 1.95+ and run:

```sh
cargo test
```

This tests the standalone recipe-checking logic only. It does not load Farmer's Delight in Horizon. No build or game test has been performed yet.

## Remaining integration

- Find a native mixin strategy supported by the exact target Nightly.
- Resolve the matching horizon-mixins revision.
- Implement block/item registration, inventory mutation, recipe data, UI, networking, persistence and content assets.
- Package and compile with the official launcher, testing alongside Minimap and TreeFall.

The example item IDs in the unit tests are illustrative; they do not assert an upstream recipe exists.
