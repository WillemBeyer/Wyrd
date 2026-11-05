# Wyrd

A native Rust magic mod for Horizon where your adventures shape your gear.
Discover item memories, experiment with alchemy, and awaken unique abilities
through exploration.

**Status: first Item Memory prototype.** In survival, an iron pickaxe can learn
both of these memories:

- **Deepslate Memory** makes it mine deepslate 15% faster.
- **Ore Memory** unlocks on the first successful harvest of a supported ore.
  Later ore breaks give a 10% chance of one extra normal resource drop after
  Fortune. Silk Touch skips the bonus.

Memories are saved on the individual pickaxe. Hover over it to see the effects.
A brief message above the hotbar announces each unlock.

In a singleplayer survival world, hold an iron pickaxe, aim at the side of
common stone at head height, and run `/wyrd tunnel`. This prototype digs a
1×2 tunnel up to three blocks deep using normal durability. It stops at ore
and other obstructions. The tooltip shows the 30-second cooldown. Any iron
pickaxe can use the prototype ability.

The Mods entry, `/wyrd` diagnostic, and block-break log are available for testing.

## Next playable goals

The Ember Path is the later full loop: a sword develops Ember Memory through
Blaze encounters, an essence helps bind it, and the player chooses Flame Burst
or Cinder Guard. See [the roadmap](docs/ROADMAP.md).

## Compatibility

This repository starts from the official
[native mod template](https://github.com/horizon-rust/horizon-native-mod-template),
not a Java/Fabric/NeoForge mod or a data-only `.hmod`.

- Mod ID: `wyrd`; initial development version: `0.1.0`.
- Rust: `1.95.0`; staging Python: `3.12+`.
- Horizon: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`
  (template pin, Nightly 2026-10-30).
- Horizon Mixins: `d9c83d7096b7ea5b83678da2efdfd6161f74997e`.
- Both pins and the original source hash are recorded in `compatibility.json`.

These are the inherited compatibility target, not a claim that Wyrd was tested
in-game or works on the latest Nightly. See [validation](VALIDATION.md).
Keep a known target until it builds and runs; upgrade it in a separate change.

## Development

See [setup instructions](docs/DEVELOPMENT.md). On Windows, use Ubuntu in WSL for
the inherited source-staging tools: they use Linux `fcntl` locks. Python packaging
works on Windows. Do not call the Linux scripts with Windows Python.

```sh
cargo fetch --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
python3 tools/package.py
```

The package command creates `target/wyrd-0.1.0.hnmod`, a source bundle, not a
precompiled game or a public release. Native code runs with the player's account
permissions. Build and play it only in a dedicated development instance/world.

## Repository layout

```text
mixins/wyrd/horizon-mixin.toml  Native manifest and the exact source target
mixins/wyrd/break.rs            Block-break diagnostic
mixins/wyrd/memory.rs           Confirmed survival mining and pickaxe memory
mixins/wyrd/ore_bonus.rs        Bonus resources after vanilla ore drops
mixins/wyrd/tunnel.rs           Server-authoritative tunnel prototype
mixins/wyrd/runtime.rs          Shared testable Rust library and injected module
compatibility.json             Pinned upstream versions and source hash
tools/horizon.py               Export, plan/apply, check and build (Linux)
tools/test_template.py         Compiled tiny-fixture integration test (Linux)
tools/test_stage.py            Full pinned-source staging and AST audit (Linux)
tools/package.py               Reproducible source packaging
tools/audit/                   Rust AST audit of the injected changes
docs/                         Architecture, setup, roadmap and decision records
```

## Licence and attribution

Wyrd keeps this repository's existing **GNU GPL version 3** licence; Cargo uses
`GPL-3.0-only`. See [LICENSE](LICENSE). The inherited template portions retain
their MIT notices in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and the
packaged source files. No Minecraft assets, exported Horizon source, game binaries,
player saves or credentials belong in this repository.

Before submitting to Horizon's registry, resolve its reviewer permission rule:
the current [mod policy](https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md)
requires an explicit written reviewer grant for GPL source. This setup does not
change your licence or issue that grant. See [the setup decision](docs/decisions/0001-native-foundation.md).
