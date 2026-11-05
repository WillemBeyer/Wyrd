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

fn show_memory_effects(
    mut after_name: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &crate::game::ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::wyrd::{
        DEEP_DELVER_KEY, DEEPSLATE_EFFECT, DEEPSLATE_MEMORY_KEY, ORE_EFFECT, ORE_MEMORY_KEY,
        TUNNEL_COOLDOWN_GROUP, TUNNEL_DEFAULT_KEY, TUNNEL_KEY_NAME, is_eligible_pickaxe,
        tunnel_cooldown_seconds,
    };
    use horizon_client::text::Text;
    use horizon_world::nbt::{JavaString, Tag};

    if !game.native_on("wyrd") || !is_eligible_pickaxe(stack.item.as_ref()) {
        return after_name;
    }
    let custom_data = stack
        .data()
        .and_then(|data| data.other("minecraft:custom_data"));
    let bound = matches!(
        custom_data,
        Some(Tag::Compound(data))
            if data.get(&JavaString::from(DEEP_DELVER_KEY)) == Some(&Tag::Byte(1))
    );
    if let Some(Tag::Compound(data)) = custom_data {
        for (key, effect) in [
            (DEEPSLATE_MEMORY_KEY, DEEPSLATE_EFFECT),
            (ORE_MEMORY_KEY, ORE_EFFECT),
        ] {
            if data.get(&JavaString::from(key)) == Some(&Tag::Byte(1)) {
                after_name.push((Text::literal(&format!("Effect: {effect}")), u32::MAX));
            }
        }
    }
    if !bound {
        return after_name;
    }
    after_name.push((Text::literal("Deep Delver"), u32::MAX));
    if game.has_singleplayer_server() {
        let key = game.native_mod_key("wyrd", TUNNEL_KEY_NAME, TUNNEL_DEFAULT_KEY);
        let label = horizon_client::keybinds::display_name(&key).plain();
        let remaining = tunnel_cooldown_seconds(game.cooldowns.percent(TUNNEL_COOLDOWN_GROUP, 0.0));
        let status = if remaining == 0 {
            format!("Tunnel Break: Ready ({label})")
        } else {
            format!("Tunnel Break: {remaining}s cooldown ({label})")
        };
        after_name.push((Text::literal(&status), u32::MAX));
    }
    after_name
}
