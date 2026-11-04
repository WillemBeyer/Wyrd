# Wyrd

A native Rust magic mod for Horizon where your adventures shape your gear.
Discover item memories, experiment with alchemy, and awaken unique abilities
through exploration.

**Status: development scaffold, not a playable magic release.** The initial bundle
registers a native Mods entry, provides `/wyrd` diagnostics, and logs block-break attempts. It does not award resonance, change drops, or
implement the abilities described below. A HEAD hook observes attempts, including
attempts the game may later reject; it must not become a progression trigger.

## First playable goal

The Ember Path: one sword develops Ember Memory through varied meaningful
interactions, one essence helps bind it, and the player chooses Flame Burst or
Cinder Guard. Start by proving an item's memory survives save/reload, dropping,
picking up, and transferring it. See [the roadmap](docs/ROADMAP.md).

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
mixins/wyrd/break.rs            Thin game hook adapter
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
