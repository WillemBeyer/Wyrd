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

// The pinned game's sole caller reaches this after a successful non-creative break.
fn remember_mined_block(game: &mut crate::game::Game<'_>, block: &str) {
    use crate::game::wyrd::{
        self, DEEPSLATE_EFFECT, DEEPSLATE_MEMORY_KEY, IRON_EFFECT, IRON_MEMORY_KEY,
    };
    use horizon_world::nbt::Tag;

    if !game.native_on("wyrd") {
        return;
    }
    let (memory_key, memory_name, effect) = match block {
        "minecraft:deepslate" => (DEEPSLATE_MEMORY_KEY, "Deepslate Memory", DEEPSLATE_EFFECT),
        ore if wyrd::is_iron_ore(ore) => (IRON_MEMORY_KEY, "Iron Memory", IRON_EFFECT),
        _ => return,
    };
    // Match the selected inventory slot used by Horizon's tool-wear method.
    let Some(stack) = game.inventory[game.selected].as_mut() else {
        return;
    };
    if stack.item.as_ref() != "minecraft:iron_pickaxe" {
        return;
    }

    let data = stack.data_mut();
    let mut custom_data = match data.other("minecraft:custom_data") {
        None => Default::default(),
        Some(Tag::Compound(entries)) => entries.clone(),
        Some(_) => return,
    };
    let key = memory_key.into();
    if custom_data.contains_key(&key) {
        return;
    }

    let mut lore = match data.other("minecraft:lore") {
        None => Vec::new(),
        Some(Tag::List(lines)) => lines.clone(),
        Some(_) => return,
    };
    let line = Tag::String(memory_name.into());
    if !lore.contains(&line) {
        if lore.len() >= 256 {
            return;
        }
        lore.push(line);
    }

    custom_data.insert(key, Tag::Byte(1));
    data.set_other("minecraft:custom_data", Some(Tag::Compound(custom_data)));
    data.set_other("minecraft:lore", Some(Tag::List(lore)));
    let message = format!("{memory_name} unlocked: {effect}");
    if game.lan_acting.is_some() {
        use horizon_protocol::packets::play::{Clientbound, Text};
        game.guest_packets.push(Clientbound::SetActionBarText(Text {
            text: Tag::String(message.as_str().into()),
        }));
    } else {
        game.titles
            .set_overlay(horizon_client::text::Text::literal(&message), false);
    }
    log::info!(target: "horizon::wyrd", "[wyrd] Iron pickaxe gained {memory_name}");
}
