// MIT License
//
// Copyright (c) 2026 Horizon Veinminer contributors
// Copyright (c) 2026 Horizon native mod contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

fn fire_tunnel(game: &mut crate::game::Game<'_>) {
    use crate::game::wyrd::{
        DEEP_DELVER_KEY, TUNNEL_COOLDOWN_GROUP, TUNNEL_COOLDOWN_TICKS, clear_wear_attempt,
        is_eligible_pickaxe, is_tunnel_stone, ore_resource, take_pending_tunnel, take_wear_attempt,
        tunnel_plan,
    };
    use horizon_protocol::packets::play::{Clientbound, Text as ActionBarText};
    use horizon_world::nbt::{JavaString, Tag};
    use std::sync::Arc;

    fn notify(game: &mut crate::game::Game<'_>, message: &str) {
        game.guest_packets
            .push(Clientbound::SetActionBarText(ActionBarText {
                text: Tag::String(message.into()),
            }));
    }

    let Some(pending) = take_pending_tunnel() else {
        return;
    };
    if game.lan_acting.is_none()
        || game.serving.as_ref().map(|serving| serving.owner) != game.lan_acting_uuid
        || game.mode_id() != 0
    {
        return;
    }
    let bound = {
        let Some(stack) = game.held() else {
            return;
        };
        is_eligible_pickaxe(stack.item.as_ref())
            && matches!(
                stack
                    .data()
                    .and_then(|data| data.other("minecraft:custom_data")),
                Some(Tag::Compound(data))
                    if data.get(&JavaString::from(DEEP_DELVER_KEY)) == Some(&Tag::Byte(1))
            )
    };
    if !bound {
        return;
    }
    let has_deep_delver = |game: &crate::game::Game<'_>| {
        let Some(stack) = game.held() else {
            return false;
        };
        is_eligible_pickaxe(stack.item.as_ref())
            && matches!(
                stack
                    .data()
                    .and_then(|data| data.other("minecraft:custom_data")),
                Some(Tag::Compound(data))
                    if data.get(&JavaString::from(DEEP_DELVER_KEY)) == Some(&Tag::Byte(1))
            )
    };

    let mut broken = 0;
    let mut skipped_ore = false;
    'tunnel: for slice in tunnel_plan(pending.pos, pending.look) {
        let mut matching = Vec::new();
        let mut obstructed = false;
        for [x, y, z] in slice {
            if !game.border.contains(f64::from(x) + 0.5, f64::from(z) + 0.5) {
                obstructed = true;
                continue;
            }
            let state = game.world.block(x, y, z);
            let kind = {
                let facts = game.res.facts();
                if facts.is_air(state) {
                    0
                } else {
                    let block = facts.get(state);
                    let name = block.name.as_ref();
                    if ore_resource(name).is_some() || name == "minecraft:ancient_debris" {
                        1
                    } else if !is_tunnel_stone(name) {
                        2
                    } else if name == pending.target.as_str() {
                        3
                    } else {
                        4
                    }
                }
            };
            match kind {
                1 => skipped_ore = true,
                2 => obstructed = true,
                3 => matching.push(([x, y, z], state)),
                _ => {}
            }
        }
        if obstructed {
            break;
        }
        for ([x, y, z], state) in matching {
            if !has_deep_delver(game) || game.world.block(x, y, z) != state {
                break 'tunnel;
            }
            clear_wear_attempt();
            let result = game.break_block([x, y, z]);
            if game.world.block(x, y, z) == state {
                break 'tunnel;
            }
            broken += 1;
            if !take_wear_attempt() {
                game.damage_held(1);
            }
            if result.is_err() {
                break 'tunnel;
            }
        }
    }
    if broken == 0 {
        let message = if skipped_ore {
            "Tunnel Break found only ore."
        } else {
            "Tunnel Break found no matching blocks."
        };
        notify(game, message);
        return;
    }

    let group = Arc::<str>::from(TUNNEL_COOLDOWN_GROUP);
    game.cooldowns
        .add(Arc::clone(&group), TUNNEL_COOLDOWN_TICKS);
    game.guest_cooldowns.push((group, TUNNEL_COOLDOWN_TICKS));
    let message = if skipped_ore {
        format!("Tunnel Break mined {broken} blocks, leaving ore intact.")
    } else {
        format!("Tunnel Break mined {broken} blocks.")
    };
    notify(game, &message);
}
