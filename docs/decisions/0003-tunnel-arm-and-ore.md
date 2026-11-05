# 0003 - Tunnel Break arming, look direction and ore bypass

Date: 2026-11-05
Status: prototype implemented; game playtest pending.

Pressing the Tunnel Break key now toggles an armed state on the integrated
server instead of mining immediately. The armed state is consumed by the next
successful stone break, so the tunnel starts from the block the player mined.
Pressing the key again disarms. Arming is rejected while the ability is on
cooldown, and the cooldown starts only when a tunnel actually mines a block.

The tunnel direction follows the dominant component of the player's look vector
at the block break: forward/back, up/down, or east/west. The same 3x3x6 volume
is used, with the cross-section perpendicular to that axis. A vertical tunnel is
a 3x3 horizontal slice repeated upward or downward.

Within each slice the tunnel mines only the exact block type that was mined.
Ore blocks are skipped and left intact, and the tunnel continues to deeper
slices instead of stopping. Other non-stone blocks still stop the slice, as
before, so dirt, gravel and similar obstructions are not bypassed.

The arm state and the pending break are thread-local on the integrated server
and are never mirrored on the client. The arming command keeps the existing
singleplayer owner, Survival and bound-pickaxe checks; the mine-to-fire hook
repeats those checks before breaking anything.

Every tunnel block must cost one normal tool-wear attempt. A HEAD hook marks
when `damage_held` actually runs during a block break; if a recursive tunnel
break did not reach it, Tunnel Break calls `damage_held(1)` for that block.
This keeps Unbreaking, breakage and the item break sound on the same path
instead of adding a second unconditional wear charge.
