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

fn scan_tunnel_trigger(game: &mut crate::game::Game<'_>, pos: [i32; 3]) {
    use crate::game::wyrd::{
        DEEP_DELVER_KEY, clear_pending_tunnel, is_eligible_pickaxe, is_tunnel_stone,
        set_pending_tunnel, set_tunnel_armed, take_tunnel_armed, tunnel_armed,
    };
    use horizon_protocol::packets::play::{Clientbound, Text as ActionBarText};
    use horizon_world::nbt::{JavaString, Tag};

    fn notify(game: &mut crate::game::Game<'_>, message: &str) {
        game.guest_packets
            .push(Clientbound::SetActionBarText(ActionBarText {
                text: Tag::String(message.into()),
            }));
    }

    if !tunnel_armed() {
        clear_pending_tunnel();
        return;
    }
    if game.lan_acting.is_none()
        || game.serving.as_ref().map(|serving| serving.owner) != game.lan_acting_uuid
    {
        return;
    }
    if game.mode_id() != 0 {
        set_tunnel_armed(false);
        return;
    }
    let eligible = {
        let Some(stack) = game.held() else {
            set_tunnel_armed(false);
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
    if !eligible {
        set_tunnel_armed(false);
        notify(
            game,
            "Tunnel Break needs a bound Deep Delver pickaxe in Survival.",
        );
        return;
    }
    let state = game.world.block(pos[0], pos[1], pos[2]);
    let name = game.res.facts().get(state).name.as_ref().to_owned();
    if !is_tunnel_stone(&name) {
        set_tunnel_armed(false);
        notify(game, "Tunnel Break needs common stone.");
        return;
    }
    let Some((_, look)) = game.aim() else {
        set_tunnel_armed(false);
        return;
    };
    take_tunnel_armed();
    set_pending_tunnel(pos, look, name);
}
