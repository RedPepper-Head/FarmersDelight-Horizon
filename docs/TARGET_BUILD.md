# Target installation: user's Horizon launcher

Observed from the launcher screenshot supplied on 2026-11-09:
- Channel: Nightly
- Build date: 2026-11-08
- Horizon source short revision: `fdc87ae`
- Native mods enabled: Minimap and TreeFall

## Compatibility policy

Target the installed Nightly first. Resolve `fdc87ae` to its **full 40-character commit** before writing any `horizon-native.json` or `horizon-mixin.toml` pin. Obtain the matching horizon-mixins revision from that build or its native build metadata; do not copy the template's older October pin blindly.

Preserve the user's existing mods and worlds. Native mods require a rebuilt Horizon client. Test in a separate instance/profile if the launcher permits.

## Next deliverable

A minimal source mixin and a correctly pinned `.hnmod` archive that can be installed via the official launcher. This is not yet implemented or tested.

Reference: https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md
