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

fn tunnel_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::wyrd::{
        TUNNEL_COOLDOWN_GROUP, TUNNEL_COOLDOWN_TICKS, tunnel_cooldown_seconds,
    };
    use horizon_client::text::Text;
    use std::sync::Arc;

    // A connected client forwards the command. The integrated server runs it
    // in its owner's player context so block breaks, drops and wear stay authoritative.
    if *ran || command.trim() != "wyrd tunnel" || !game.native_on("wyrd") || game.remote.is_some() {
        return;
    }
    if !game.lan.as_ref().is_some_and(|host| {
        game.lan_acting_uuid.is_some() && game.lan_acting_uuid == host.hub.singleplayer_owner
    }) {
        return;
    }

    *ran = true;
    let has_pickaxe = |game: &crate::game::Game<'_>| matches!(game.held(), Some(stack) if stack.item.as_ref() == "minecraft:iron_pickaxe");
    if game.mode_id() != 0 || !has_pickaxe(game) {
        game.system_message(Text::literal(
            "Tunnel Break needs an iron pickaxe in Survival.",
        ));
        return;
    }
    let remaining = tunnel_cooldown_seconds(game.cooldowns.percent(TUNNEL_COOLDOWN_GROUP, 0.0));
    if remaining > 0 {
        game.system_message(Text::literal(&format!(
            "Tunnel Break is ready in {remaining}s."
        )));
        return;
    }

    let Some(hit) = game
        .aim()
        .and_then(|(eye, dir)| game.clip_ray(eye, dir, game.reach()))
    else {
        game.system_message(Text::literal("Aim at the side of a stone block."));
        return;
    };
    if hit.face[1] != 0 || hit.face == [0, 0, 0] {
        game.system_message(Text::literal("Aim at the side of a stone block."));
        return;
    }

    let mut broken = 0;
    'tunnel: for depth in 0..3 {
        let x = hit.pos[0] - depth * hit.face[0];
        let z = hit.pos[2] - depth * hit.face[2];
        if !game.border.contains(f64::from(x) + 0.5, f64::from(z) + 0.5) {
            break;
        }
        let positions = [[x, hit.pos[1], z], [x, hit.pos[1] - 1, z]];
        let states = positions.map(|[x, y, z]| game.world.block(x, y, z));
        let facts = game.res.facts();
        let air = states.map(|state| facts.is_air(state));
        // Inspect both blocks first so an ore vein or obstruction leaves the
        // whole slice intact. Empty slices also end the tunnel.
        if air == [true, true]
            || states.iter().zip(air).any(|(&state, air)| {
                !air && !matches!(
                    facts.get(state).name.as_ref(),
                    "minecraft:stone"
                        | "minecraft:deepslate"
                        | "minecraft:granite"
                        | "minecraft:diorite"
                        | "minecraft:andesite"
                        | "minecraft:tuff"
                        | "minecraft:calcite"
                        | "minecraft:netherrack"
                        | "minecraft:blackstone"
                        | "minecraft:basalt"
                        | "minecraft:smooth_basalt"
                        | "minecraft:end_stone"
                )
            })
        {
            break;
        }
        for ([x, y, z], air) in positions.into_iter().zip(air) {
            if air {
                continue;
            }
            if !has_pickaxe(game) {
                break 'tunnel;
            }
            let before = game.world.block(x, y, z);
            let result = game.break_block([x, y, z]);
            if game.world.block(x, y, z) == before {
                break 'tunnel;
            }
            broken += 1;
            if result.is_err() {
                break 'tunnel;
            }
        }
    }
    if broken == 0 {
        game.system_message(Text::literal("Tunnel Break stopped. Aim at common stone."));
        return;
    }

    let group = Arc::<str>::from(TUNNEL_COOLDOWN_GROUP);
    game.cooldowns
        .add(Arc::clone(&group), TUNNEL_COOLDOWN_TICKS);
    game.guest_cooldowns.push((group, TUNNEL_COOLDOWN_TICKS));
    game.system_message(Text::literal(&format!(
        "Tunnel Break mined {broken} blocks."
    )));
}
