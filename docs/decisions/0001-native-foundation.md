# 0001 — Native Horizon foundation

Date: 2026-11-04
Status: adopted for scaffolding; compatibility still needs Wyrd build/playtest evidence.

Use the official native source-mixin template at
`cc8ce2d896782099abfa9f0abf26fa80c9e4bddb` as the initial known layout.
Keep its recorded Horizon/Mixins pins instead of silently moving to current main.
Newer trait APIs remain a future evaluation, not a dependency assumed here.

Keep one repository and a shared testable runtime with thin hook adapters. The
starter hook only logs block-break attempts and is not the item-memory mechanic.
Use Linux/WSL for the template staging tools; Windows packaging is supported by
the Python packager, but this does not establish Windows game-build compatibility.

Preserve the repository's existing GPLv3 licence. Retain the upstream MIT notices.
Horizon's current register policy requires a written reviewer grant for GPL code;
resolve that with the project owner before submission. No reviewer grant or
relicensing is performed by this setup.

Sources:
- https://github.com/horizon-rust/horizon-native-mod-template
- https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md
- https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md
