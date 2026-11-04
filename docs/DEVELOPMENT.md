# Development setup

## Windows and Linux

Use Windows for the Horizon launcher and editor, and Ubuntu/WSL for the upstream
template's Linux-only staging/check scripts. The existing Windows repository can
be reached in WSL as `/mnt/c/Users/conta/Documents/Github/mc modding/Wyrd`.
Always quote that path: it contains spaces. Full builds may be faster on WSL's
Linux filesystem; keep generated checkouts/build outputs outside the repository.

Install Git, Python 3.12+, a C toolchain, and Rust through the official rustup
installer. `rust-toolchain.toml` selects Rust 1.95.0, rustfmt and clippy. Do not
commit a Rust toolchain or downloaded dependencies. GitHub Actions provides the
Linux environment for the formatting, lint, runtime and compiled-fixture checks.

## Fetch tools and dependencies

From the repository in Ubuntu/WSL:

```sh
git clone https://github.com/horizon-rust/horizon ../horizon
git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
git -C ../horizon-mixins checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e
cargo fetch --locked --manifest-path ../horizon-mixins/Cargo.toml
cargo fetch --locked
```

Keep both checkouts separate from Wyrd. The Horizon history must contain the
commit in `compatibility.json`. Set `HORIZON_MIXINS` to use a different checkout
location. The original upstream checkouts and player resources are never rewritten.

## Fast checks and packaging

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
python3 tools/test_template.py
python3 tools/package.py
```

The fixture compiles the actual bundle against a minimal host, checks the
original method still executes once with its result preserved, audits the Rust
AST, and rejects incompatible source revisions/targets. It is not a full game test.
The packager checks that the bundle ID and both Horizon pins agree, and writes
a deterministic source archive with a SHA-256. On Windows use `python tools/package.py`.

## Full pinned-source checks

```sh
python3 tools/test_stage.py --horizon ../horizon
python3 tools/horizon.py stage --horizon ../horizon
cargo fetch --locked --manifest-path target/mixins/modded/Cargo.toml
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
```

Full checks need the pinned Horizon source, its dependencies, and native build
libraries. They are required before calling a changed bundle game-compatible.
Generated sources, AST plans and binaries stay under ignored `target/` directories.
The template's build disables optional audio/controller features; do not describe
its binary as a fully tested player release.

## Play test

With a compatible Windows Horizon launcher already installed, the documented
local bundle route is:

```powershell
horizon-launcher native-build "C:\Users\conta\Documents\Github\mc modding\Wyrd\mixins\wyrd" --trust-native-code
```

Check the installed launcher's `--help` for its instance options and select a
dedicated Wyrd development instance. This setup does not run that command or
touch your worlds. The launcher uses its own managed Rust toolchain. Check the
built game's logs for `[wyrd] Breaking block at ...`. Do not test with valued saves.

## Significant changes

Use a focused commit for each meaningful milestone. Run relevant checks, update
`VALIDATION.md` and `CHANGELOG.md`, then push. Keep full game-build/playtest claims
separate from fixture or unit-test results. Published versions are immutable;
increase the manifest/Cargo versions together before a new release.
