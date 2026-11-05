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
        DEEP_DELVER_KEY, TUNNEL_COOLDOWN_GROUP, is_eligible_pickaxe, set_tunnel_armed,
        tunnel_armed, tunnel_cooldown_seconds,
    };
    use horizon_client::text::Text;
    use horizon_world::nbt::{JavaString, Tag};

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
    let has_deep_delver = |game: &crate::game::Game<'_>| {
        let Some(stack) = game.held() else {
            return false;
        };
        if !is_eligible_pickaxe(stack.item.as_ref()) {
            return false;
        }
        matches!(
            stack
                .data()
                .and_then(|data| data.other("minecraft:custom_data")),
            Some(Tag::Compound(data))
                if data.get(&JavaString::from(DEEP_DELVER_KEY)) == Some(&Tag::Byte(1))
        )
    };
    if game.mode_id() != 0 || !has_deep_delver(game) {
        game.system_message(Text::literal(
            "Tunnel Break needs a bound Deep Delver pickaxe in Survival.",
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

    if tunnel_armed() {
        set_tunnel_armed(false);
        game.system_message(Text::literal("Tunnel Break disarmed."));
    } else {
        set_tunnel_armed(true);
        game.system_message(Text::literal("Tunnel Break armed. Mine a block to fire."));
    }
}
