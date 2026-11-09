# Farmer's Delight → Horizon: porting plan

Status: planning; no gameplay functionality has been implemented or tested yet.

## Goal
Reimplement the behavior and content of Farmer's Delight Refabricated for Horizon, aiming for feature parity rather than a reduced subset.

## Upstream references
- Horizon: https://github.com/horizon-rust/horizon
- Fabric source: https://github.com/macuguita/FarmersDelightRefabricated

## Workstreams
1. Audit upstream source, resources, data definitions, dependencies, and licensing. Pin exact upstream commits.
2. Confirm current Horizon mod APIs, packaging, registries, lifecycle hooks, asset formats, networking and persistence.
3. Port items, food, blocks, recipes, loot tables, tags, models, textures, translations and sounds where permitted.
4. Implement crop growth, harvesting, soil interactions and world generation.
5. Implement knives, cutting board and other interactive blocks.
6. Implement cooking pot inventory, recipe processing, UI, sync and saved state.
7. Implement effects, advancements, villager/trading and cross-system integrations.
8. Run feature-parity tests in singleplayer and multiplayer; profile performance; package releases.

## Acceptance criteria
- Every feature in the pinned upstream version is recorded in a parity matrix.
- Each ported feature has an in-game test and expected behavior.
- Native mod loads on a pinned Horizon build without Java/Fabric.
- Licensing and attribution are reviewed before redistributing assets.
- Unsupported Horizon capabilities are tracked explicitly, not silently omitted.

## Next technical task
Inspect Horizon's current native mod examples and manifest format, then create the minimal compiling and loading native mod. Do not assume API names or file formats without verifying against the pinned engine commit.
