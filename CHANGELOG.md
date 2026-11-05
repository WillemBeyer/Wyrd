# Changelog

## Unreleased

- Preserve pickaxe memories during inventory sync so the tooltip can show their effects in singleplayer and LAN.

- Let iron pickaxes learn Iron Memory for a 20% chance of one extra raw iron on later iron ore breaks without Silk Touch.
- Show the effects of unlocked memories in the pickaxe tooltip.

- Give remembered iron pickaxes 15% faster deepslate mining and announce first unlocks above the hotbar.

- Give the exact iron pickaxe used to mine deepslate a persistent Deepslate Memory and tooltip line.

- Use the incremental performance profile on Windows to avoid a title-screen startup stack overflow in the pinned dev build.

- Register Wyrd in Options > Mods, add startup and /wyrd diagnostics, and provide a direct development-instance launch shortcut.

- Send block-break test messages through Horizon's INFO logger instead of raw stderr.

- Include Rust standard-library sources in the developer toolchain and document VS Code setup.

- Add a Windows rebuild shortcut targeting Wyrd Dev with the incremental performance profile.

- Bootstrap Wyrd from Horizon's official native source-mixin template.
- Rename the bundle and standalone runtime to `wyrd`; retain exact upstream pins.
- Add Linux CI, source packaging checks, project documentation and attribution.
- Preserve the existing GPLv3 project licence.

No item memory, alchemy, abilities or custom GUI is implemented in this scaffold.
