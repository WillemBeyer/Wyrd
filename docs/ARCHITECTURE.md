# Architecture

The initial foundation deliberately keeps the upstream template's shape.

- `runtime.rs` holds plain Rust rules that can run without Horizon. It is the
  standalone library and the exact injected game module, so tests exercise the
  same source. No engine dependencies or generated runtime copies.
- `break.rs` is a thin typed adapter. It currently logs an attempted action only.
- `horizon-mixin.toml` declares the module and HEAD injection. All game changes
  go through horizon-mixins plan/apply and its source audit.

Prefer documented stable native hooks when adding actual gameplay. Replacing the
template's direct internal hook with supported gameplay events is a separate
capability/compatibility milestone. Do not build progression on block-break
attempts or poll every item/entity each frame.

## Planned boundaries

1. Discovery rules consume accepted semantic events, not render ticks or raw kill counts.
2. Item memory belongs to the particular item stack; player knowledge is separate.
3. Version saved data and plan migration before writing long-lived progression.
4. The authoritative server validates progression, catalyst consumption and binding;
   client UI only requests choices and displays confirmed state, even in singleplayer.
5. UI, sounds and particles remain adapters over those rules.

Do not create a generic magic framework or multiple crates until the first
playable loop needs them. First prove persistence through save/reload, dropped
items, chests and transfers, without duplication or accidentally shared memories.
