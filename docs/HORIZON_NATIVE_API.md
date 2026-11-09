# Verified Horizon native mod integration

Checked against horizon-rust/horizon at commit `3b691ea10b5fbbc90e00cb24e96a1c841601bcfb`.

Sources:
- https://github.com/horizon-rust/horizon/blob/3b691ea10b5fbbc90e00cb24e96a1c841601bcfb/docs/native-mods.md
- https://github.com/horizon-rust/horizon/blob/3b691ea10b5fbbc90e00cb24e96a1c841601bcfb/docs/api/mod-files.md
- https://github.com/horizon-rust/horizon-native-mod-template

## Confirmed packaging

Native mods are Rust source mixin bundles applied **before Horizon compiles** using horizon-mixins. A native-only `.hnmod` is a ZIP with:
- `horizon-native.json` (schema 1, id, version, game, Horizon commit, horizon-mixins commit, optional hook API version).
- `mixins/<id>/horizon-mixin.toml`.
- Rust helpers and allowed supporting files under that mixin folder.

A content-only `.hmod` instead has `horizon-mod.toml` plus `assets/` and `data/`. The docs also describe combined content/native packaging in a `.hmod`, subject to launcher support.

Native mods are not independently loaded shared libraries: Horizon must be rebuilt with the mod. Never ship fabricated commit pins or claim a bundle is tested without compiling it.

## Useful confirmed hooks

- Native API 13: item use and use-on-block.
- Native API 14: block break and placement.
- Native API 15: custom block entities and ticking.
- Native API 16: block entity item containers.
- Native API 17: menus for mod containers.
- Native API 18: neighboring mod block entity and block state property updates.

These are promising foundations for Farmer's Delight cooking pot, cutting board, and crop interactions, but do not prove full feature parity.

## Implementation gate

Before creating the first executable native bundle:
1. Read the **current** `horizon-native-mod-template` and pin its compatible Horizon and horizon-mixins commits.
2. Select a target Horizon build and confirm its supported hooks.
3. Create the smallest valid bundle and verify the launcher rebuilds and runs it.
4. Record the build command and actual results in this repository.
5. Add gameplay incrementally; run tests for each feature.

No Rust bundle has been compiled or run as part of this documentation update.
