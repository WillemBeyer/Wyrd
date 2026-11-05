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

fn mark_deepslate_boost(game: &crate::game::Game<'_>, pos: [i32; 3]) {
    use crate::game::wyrd::{DEEPSLATE_MEMORY_KEY, is_eligible_pickaxe, set_deepslate_boost};
    use horizon_world::nbt::{JavaString, Tag};

    let active = game.native_on("wyrd")
        && game.held().is_some_and(|stack| {
            is_eligible_pickaxe(stack.item.as_ref())
                && matches!(
                    stack
                        .data()
                        .and_then(|data| data.other("minecraft:custom_data")),
                    Some(Tag::Compound(data))
                        if data.get(&JavaString::from(DEEPSLATE_MEMORY_KEY))
                            == Some(&Tag::Byte(1))
                )
        })
        && game
            .res
            .facts()
            .get(game.world.block(pos[0], pos[1], pos[2]))
            .name
            .as_ref()
            == "minecraft:deepslate";
    set_deepslate_boost(active);
}
