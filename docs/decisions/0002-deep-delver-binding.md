# 0002 - Deep Delver binding and Tunnel Break keybind

Date: 2026-11-05
Status: prototype implemented; pinned source plan/apply passes, game playtest pending.

Bind exactly one eligible pickaxe (iron, diamond or netherite) as Deep Delver
when the singleplayer owner sneak-uses an enchanting table with both memories in
the main hand and an amethyst shard in the offhand. Validate the whole recipe
before mutating either item, consume one shard, and store only
`wyrd:deep_delver` in `minecraft:custom_data` so memories and unrelated item data
stay intact. Run the binding on the integrated server via the existing
`use_hand` action path and require `serving.owner == lan_acting_uuid`.

Register Tunnel Break as a native mod key (default `key.keyboard.v`) through
`NativeMod.keys`, and handle it in the existing `native_key` hook. The key
handler sends the existing `wyrd tunnel` command internally through
`Serverbound::ChatCommand`; the server still checks Survival, the eligible tier
and the binding marker. This keeps one authoritative implementation and removes
the old unbound-pickaxe prototype access. `/wyrd` diagnostics remain local.

The pinned mixin tool rejects non-HEAD injections into functions containing
closures. At the pin, `Game::destroy_progress` contains
`self.player.as_ref().is_some_and(|p| ...)`, `break_block_as_player` and
`draw_stack_tooltip_with` contain closures, and `block_loot_at` contains macros
whose identity the auditor cannot resolve. The Deepslate bonus therefore uses a
one-shot thread-local set by HEAD hooks at the two speed callers and consumed by
a RETURN on the closure-free `modify_destroy_speed`. The ore bonus arms in a HEAD
hook on `break_block_as_player` and adds its extra drop in a HEAD hook on
`pop_resource`. Tooltip lines are added with MODIFY_ARG at the closure-free
`draw_stack_tooltip` wrapper. The new interaction and keypress use supported HEAD
hooks; no string-based source rewriting is used.
