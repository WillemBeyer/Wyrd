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

// This loot path runs only after a successful survival harvest. Its held snapshot
// predates any memory earned on the current break.
fn add_iron_memory_drop(
    game: &mut crate::game::Game<'_>,
    state: crate::game::BlockStateId,
    held: Option<&crate::game::ItemStack>,
    pos: [i32; 3],
) {
    use crate::game::wyrd::{IRON_BONUS_CHANCE, IRON_MEMORY_KEY, is_iron_ore};
    use horizon_world::nbt::{JavaString, Tag};

    let Some(stack) = held else {
        return;
    };
    if !game.native_on("wyrd")
        || !game.ui.rules.bool("block_drops")
        || stack.item.as_ref() != "minecraft:iron_pickaxe"
        || stack.enchantment_level("minecraft:silk_touch") > 0
        || !is_iron_ore(game.res.facts().get(state).name.as_ref())
    {
        return;
    }

    let Some(Tag::Compound(data)) = stack
        .data()
        .and_then(|data| data.other("minecraft:custom_data"))
    else {
        return;
    };
    if data.get(&JavaString::from(IRON_MEMORY_KEY)) == Some(&Tag::Byte(1))
        && game.world.random.next_float() < IRON_BONUS_CHANCE
    {
        game.pop_resource(pos, crate::game::ItemStack::new("minecraft:raw_iron", 1));
    }
}
