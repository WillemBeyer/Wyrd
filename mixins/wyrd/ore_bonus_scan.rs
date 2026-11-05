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

fn scan_ore_bonus(game: &mut crate::game::Game<'_>, pos: [i32; 3]) {
    use crate::game::wyrd::{
        ORE_MEMORY_KEY, is_eligible_pickaxe, ore_resource, set_ore_bonus_resource,
    };
    use horizon_world::nbt::{JavaString, Tag};

    let resource = (|| {
        if !game.native_on("wyrd") || !game.ui.rules.bool("block_drops") {
            return None;
        }
        let stack = game.held()?;
        if !is_eligible_pickaxe(stack.item.as_ref())
            || stack.enchantment_level("minecraft:silk_touch") > 0
        {
            return None;
        }
        let has_memory = matches!(
            stack
                .data()
                .and_then(|data| data.other("minecraft:custom_data")),
            Some(Tag::Compound(data))
                if data.get(&JavaString::from(ORE_MEMORY_KEY)) == Some(&Tag::Byte(1))
        );
        if !has_memory {
            return None;
        }
        let state = game.world.block(pos[0], pos[1], pos[2]);
        let name = game.res.facts().get(state).name.clone();
        let resource = ore_resource(name.as_ref())?;
        game.res
            .items()
            .correct_for_drops(game.held(), &name)
            .then_some(resource)
    })();
    set_ore_bonus_resource(pos, resource);
}
