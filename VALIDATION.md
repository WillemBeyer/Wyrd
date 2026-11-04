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
