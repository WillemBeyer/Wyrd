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

fn try_bind_deep_delver(game: &mut crate::game::Game<'_>, hit: Option<&crate::game::Hit>) {
    use crate::game::wyrd::{
        DEEP_DELVER_KEY, DEEPSLATE_MEMORY_KEY, ORE_MEMORY_KEY, is_eligible_pickaxe,
    };
    use horizon_client::first_person::Hand;
    use horizon_protocol::packets::play::{Clientbound, Text as ActionBarText};
    use horizon_world::nbt::{JavaString, Tag};
    use winit::keyboard::KeyCode;

    fn notify(game: &mut crate::game::Game<'_>, message: &str) {
        game.guest_packets
            .push(Clientbound::SetActionBarText(ActionBarText {
                text: Tag::String(message.into()),
            }));
    }

    if !game.native_on("wyrd")
        || game.hand != Hand::Main
        || !game.keys.contains(&KeyCode::ShiftLeft)
        || game.lan_acting.is_none()
        || game.serving.as_ref().map(|serving| serving.owner) != game.lan_acting_uuid
        || game.mode_id() != 0
    {
        return;
    }
    let Some(hit) = hit else {
        return;
    };
    if game
        .res
        .facts()
        .get(game.world.block(hit.pos[0], hit.pos[1], hit.pos[2]))
        .name
        .as_ref()
        != "minecraft:enchanting_table"
    {
        return;
    }

    let problem: Option<&'static str> = {
        let Some(stack) = game.inventory[game.selected].as_ref() else {
            return;
        };
        if !is_eligible_pickaxe(stack.item.as_ref()) {
            return;
        }
        match stack
            .data()
            .and_then(|data| data.other("minecraft:custom_data"))
        {
            Some(Tag::Compound(entries)) => {
                let has_deepslate =
                    entries.get(&JavaString::from(DEEPSLATE_MEMORY_KEY)) == Some(&Tag::Byte(1));
                let has_ore = entries.get(&JavaString::from(ORE_MEMORY_KEY)) == Some(&Tag::Byte(1));
                if entries.contains_key(&JavaString::from(DEEP_DELVER_KEY)) {
                    Some("Deep Delver is already bound to this pickaxe.")
                } else if !has_deepslate || !has_ore {
                    Some("Deep Delver needs Deepslate Memory and Ore Memory.")
                } else if !game.offhand.as_ref().is_some_and(|offhand| {
                    offhand.item.as_ref() == "minecraft:amethyst_shard" && offhand.count > 0
                }) {
                    Some("Deep Delver needs an amethyst shard in the offhand.")
                } else {
                    None
                }
            }
            _ => Some("Deep Delver needs Deepslate Memory and Ore Memory."),
        }
    };
    if let Some(message) = problem {
        notify(game, message);
        return;
    }

    let Some(stack) = game.inventory[game.selected].as_mut() else {
        return;
    };
    let data = stack.data_mut();
    let mut entries = match data.other("minecraft:custom_data") {
        Some(Tag::Compound(entries)) => entries.clone(),
        _ => return,
    };
    entries.insert(JavaString::from(DEEP_DELVER_KEY), Tag::Byte(1));
    data.set_other("minecraft:custom_data", Some(Tag::Compound(entries)));

    let emptied = match game.offhand.as_mut() {
        Some(offhand) => {
            offhand.count -= 1;
            offhand.count == 0
        }
        None => false,
    };
    if emptied {
        game.offhand = None;
    }
    notify(game, "Deep Delver bound to this pickaxe.");
}
