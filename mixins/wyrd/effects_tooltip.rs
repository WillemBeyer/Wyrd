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
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &crate::game::ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::wyrd::{DEEPSLATE_EFFECT, DEEPSLATE_MEMORY_KEY, IRON_EFFECT, IRON_MEMORY_KEY};
    use horizon_client::text::Text;
    use horizon_world::nbt::{JavaString, Tag};

    if lines.is_empty()
        || !game.native_on("wyrd")
        || stack.item.as_ref() != "minecraft:iron_pickaxe"
    {
        return lines;
    }
    let Some(Tag::Compound(data)) = stack
        .data()
        .and_then(|data| data.other("minecraft:custom_data"))
    else {
        return lines;
    };
    for (key, effect) in [
        (DEEPSLATE_MEMORY_KEY, DEEPSLATE_EFFECT),
        (IRON_MEMORY_KEY, IRON_EFFECT),
    ] {
        if data.get(&JavaString::from(key)) == Some(&Tag::Byte(1)) {
            lines.push((Text::literal(&format!("Effect: {effect}")), u32::MAX));
        }
    }
    lines
}
