# Changelog

## Unreleased

- Bind a pickaxe as Deep Delver by sneak-using an enchanting table with both memories in the main hand and one amethyst shard in the offhand; the binding is one custom-data marker and survives inventory sync and relogging.
- Add a configurable Tunnel Break keybind (default V) that activates the existing 3x3, six-block Tunnel Break only for a bound eligible pickaxe; keep `/wyrd` diagnostics and use the command path internally.
- Show Deep Delver, Tunnel Break and the remaining cooldown in the bound pickaxe's tooltip.
- Extend Deepslate Memory, Ore Memory, their bonuses, inventory sync and tooltips from iron to diamond and netherite pickaxes.
- Retarget the existing bonus hooks to pinned-source-compatible HEAD, MODIFY_ARG and RETURN sites; the pinned mixin tool rejects non-HEAD injection into functions with closures.
- Preserve pickaxe memories during inventory sync so the tooltip can show their effects in singleplayer and LAN.

- Give iron pickaxes Ore Memory for a 10% chance of one extra resource on later ore breaks without Silk Touch.
- Add the singleplayer Tunnel Break prototype with normal tool wear and a 30-second cooldown.
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
