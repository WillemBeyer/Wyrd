# Validation

Wyrd does not inherit the upstream template's validation claims. The checked-in
compatibility target comes from the upstream template; it is not a claim of
Wyrd compatibility with the latest Nightly.

Setup checks performed on 2026-11-04 are recorded below after execution.
GitHub Actions runs formatting, lint, standalone tests, the actual compiled
bundle fixture/AST audit, and source packaging on Ubuntu for pushes and PRs.

Full pinned-Horizon stage/check/test and an interactive game session are separate
checks; see docs/DEVELOPMENT.md. No player resources or worlds are needed for the
fast fixture. No playable alpha or registry-ready release is claimed.

## Local setup results — 2026-11-04

- Windows / Python 3.13: two source-packaging tests passed, including deterministic bytes, bundled notices and rejection of inconsistent Horizon pins.
- Python compileall: passed for the tooling.
- Source package generated: target/wyrd-0.1.0.hnmod (development artifact, ignored by Git).
- Existing GPLv3 LICENSE is unchanged.
- Local full stage/check/test attempts stop at the inherited Linux-only fcntl import. The installed Ubuntu has Python 3.10 and no Rust on PATH; these checks were not completed locally.
- Ubuntu GitHub Actions: Rust formatting, clippy with warnings denied, standalone unit tests, the compiled bundle fixture/AST audit, packaging tests and source artifact upload all passed for setup commit `178363fb02d129d07c9f9ac0168141cffe8326af`.
- Evidence: https://github.com/WillemBeyer/Wyrd/actions/runs/41907932719 (successful run).
- Full pinned-game build and interactive playtest: not performed. No game compatibility or playable magic features are claimed.

## Windows rebuild shortcut — 2026-11-04

- Verified against installed Horizon Launcher 0.2.41 help: native-build supports --instance and --profile dev.
- Confirmed Wyrd Dev exists as instance ID wyrd-dev.
- rebuild.bat --check succeeds with the actual launcher and a repository path containing spaces; it starts no build.
- A missing-launcher check returns exit code 2 and prints recovery instructions.
- Local .wyrd-launcher.txt is ignored by Git. No game build or playtest was started for this shortcut change.

## Windows Rust editor setup — 2026-11-04

- Installed the official Rust 1.95.0 GNU developer toolchain for the user, including rust-src, rustfmt and clippy; Horizon keeps its separate managed compiler.
- Cargo metadata successfully loads the Wyrd workspace on Windows.
- cargo check, formatting, clippy with warnings denied, and workspace tests all passed on Windows.
- Existing VS Code processes need a full restart to receive the user PATH update. The editor UI was not restarted automatically.

## Visible starter logging — 2026-11-04

- Inspected the existing launcher build metadata: Wyrd 0.1.0 is included on pinned Horizon ba6344a. The initial inference about the Nightly banner was not proof of the running executable; see the direct-launch evidence below.
- The prior runtime used raw stderr, so latest.log was not a reliable place to look for the test message.
- Runtime now uses the game's existing log crate at INFO level. A regression test captures the logging record and checks its level and coordinates.
- Windows cargo check, formatting, clippy, both runtime tests and both packaging tests passed.
- Full game rebuild/playtest of this logging change still needs the development shortcut; CI covers the compiled bundle fixture separately.

## Native registration and real-game smoke test — 2026-11-04

- Setup fix commit: `f646830`; GitHub formatting, clippy, runtime tests,
  compiled three-hook fixture/AST audit and packaging checks all passed:
  https://github.com/WillemBeyer/Wyrd/actions/runs/41914077053
- The tiny fixture now exercises mod-list registration, local diagnostic command
  handling and preservation of the original block-break method, not only logging.
- Actual launcher build on Horizon `ba6344a` succeeded with the launcher's managed
  Rust 1.99.1 and Mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`.
  Installed version: `native-ba6344a-9bf0349e7331`, dev profile. Compilation of
  the changed game crate took 25.21 s; a subsequent cached build/install took
  329 s including resource copying. A completed compiler line is not a completed install.
- Windows source audit of the real pinned export and actual staged source passed:
  only the three declared HEAD statements and runtime module differed.
- Direct launcher invocation used the installed native executable (checked by
  process path), rather than relying on an already-open launcher window. Earlier
  supplied logs reported the regular Nightly; the direct native run reports
  Horizon 0.1.0. Always verify the executable and startup marker as well as metadata.
- A separate **Wyrd Diagnostic** creative world was used; existing worlds were not modified.
- Startup log: `[wyrd] Native scaffold loaded (0.1.0)` appeared on both client
  and integrated-server threads.
- `/wyrd` produced local chat and INFO output confirming the mod was enabled.
- The real player-action mining path produced
  `[wyrd] Breaking block at -22, 65, 2` on the server thread.
- Options > Mods UI reported button `wyrd-0-1-0`, label **Wyrd 0.1.0: ON**.
- The diagnostic session was saved and closed through Horizon's own quit command.

This verifies native loading, registration, diagnostics and a break-attempt hook;
it does not implement or validate item memory, magic, or compatibility with a
different Horizon source revision.

## Deep Delver binding prototype - 2026-11-05

- Added the Deep Delver enchanting-table recipe, one-marker custom-data binding,
  bound-pickaxe Tunnel Break gate, configurable keybind (default V), and the
  diamond/netherite tier extension across memories, bonuses, sync and tooltips.
- Standalone Windows checks: `cargo fmt --all --check`, `cargo check
  --workspace --all-targets --locked --offline` and `cargo clippy
  --workspace --all-targets --locked --offline -- -D warnings` passed. These
  checks cover the runtime library, not the injected mixin source.
- Pinned-source audit: built the pinned Horizon Mixins CLI at
  `d9c83d7096b7ea5b83678da2efdfd6161f74997e` and ran `plan`/`apply` against the
  pinned Horizon source at `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`.
  The plan contains 14 injections and one module; both commands returned
  `"status": "ok"`. The staged source was inspected around each generated call.
- Repo source audit: `native-mod-source-audit` passed on the same plan/stage
  after adding MODIFY_ARG support and normalizing the pinned printer's
  single-expression block/paren shapes. Result:
  `all original ASTs preserved apart from the planned hooks and support module`.
- Pinned-source limitation found: the mixin validator rejects non-HEAD
  injections into functions whose pristine bodies contain closures. At this pin,
  `Game::destroy_progress` has `self.player.as_ref().is_some_and(|p| ...)`,
  `break_block_as_player` and `draw_stack_tooltip_with` contain closures, and
  `block_loot_at` contains macros the auditor cannot resolve. The existing
  Deepslate/Ore/tooltip hooks were retargeted to HEAD/MODIFY_ARG/RETURN sites as
  described in `docs/decisions/0002-deep-delver-binding.md`; the new interaction
  and keypress use supported HEAD hooks.
- Full staged `cargo check` did not complete on this Windows host: `ring`'s build
  script requires `gcc.exe`, and no Windows C compiler is installed. This is an
  environment limitation, not a reported source error. The Linux staging check
  and the in-game playtest remain to be run by the project owner.

- Integration review updated the fixtures and package inventory for all 14
  hooks. Compile-only fixture checks, the fixture audit, pinned Horizon source
  staging and audit, and source packaging passed. The ore bonus now requires
  a matching drop at the active player break position. No game was launched.


## Windows normal-startup stack-overflow workaround — 2026-11-04

- User reported Windows status 0xc00000fd / main stack overflow when the pinned dev-profile game started normally, before Wyrd registration. The earlier world-direct smoke test missed that path.
- Rebuilt the same unchanged mod source with Horizon's supported incremental performance profile. Compiler output completed in 1m 41s; full launcher build/install completed in 251 s.
- Installed and selected version: native-ba6344a-dd53697291ee, profile performance.
- Tested the compiled executable without --world: normal title screen reached, Mods showed Wyrd 0.1.0: ON, and clean exit succeeded.
- Repeated the test through the launcher using the installed performance instance without --world: title screen, Mods entry and exit code 0 all passed. No stack overflow occurred.
- rebuild.bat now uses --profile performance. Both batch-file --check paths passed. No Wyrd gameplay logic or world files changed for this fix.
- This is a verified profile workaround for the pinned engine; it does not identify the exact upstream stack frame or establish dev-profile safety.
