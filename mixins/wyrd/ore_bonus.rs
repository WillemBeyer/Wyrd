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

// The held snapshot predates the memory awarded on the current break.
// The original loot vector already contains vanilla's Fortune results.
fn add_ore_memory_drop(
    mut drops: Vec<ItemStack>,
    game: &mut crate::game::Game<'_>,
    state: BlockStateId,
    held: &Option<ItemStack>,
) -> Vec<ItemStack> {
    use crate::game::wyrd::{ORE_BONUS_CHANCE, ORE_MEMORY_KEY, ore_resource};
    use horizon_world::nbt::{JavaString, Tag};

    let Some(stack) = held else {
        return drops;
    };
    if !game.native_on("wyrd")
        || !game.ui.rules.bool("block_drops")
        || stack.item.as_ref() != "minecraft:iron_pickaxe"
        || stack.enchantment_level("minecraft:silk_touch") > 0
    {
        return drops;
    }

    let Some(resource) = ore_resource(game.res.facts().get(state).name.as_ref()) else {
        return drops;
    };
    if !drops
        .iter()
        .any(|drop| drop.item.as_ref() == resource && drop.count > 0)
    {
        return drops;
    }
    let Some(Tag::Compound(data)) = stack
        .data()
        .and_then(|data| data.other("minecraft:custom_data"))
    else {
        return drops;
    };
    if data.get(&JavaString::from(ORE_MEMORY_KEY)) == Some(&Tag::Byte(1))
        && game.world.random.next_float() < ORE_BONUS_CHANCE
    {
        drops.push(ItemStack::new(resource, 1));
    }
    drops
}
