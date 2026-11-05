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

// Both the local prediction and the integrated server use this progress value.
fn boost_deepslate_progress(progress: f32, game: &crate::game::Game<'_>, pos: [i32; 3]) -> f32 {
    use crate::game::wyrd::{DEEPSLATE_MEMORY_KEY, DEEPSLATE_SPEED_MULTIPLIER};
    use horizon_world::nbt::Tag;

    if progress <= 0.0 || !progress.is_finite() || !game.native_on("wyrd") {
        return progress;
    }
    let Some(stack) = game.held() else {
        return progress;
    };
    if stack.item.as_ref() != "minecraft:iron_pickaxe" {
        return progress;
    }
    let Some(Tag::Compound(data)) = stack.data().and_then(|data| data.other("minecraft:custom_data")) else {
        return progress;
    };
    let key = horizon_world::nbt::JavaString::from(DEEPSLATE_MEMORY_KEY);
    if data.get(&key) != Some(&Tag::Byte(1)) {
        return progress;
    }
    let state = game.world.block(pos[0], pos[1], pos[2]);
    if game.res.facts().get(state).name.as_ref() == "minecraft:deepslate" {
        progress * DEEPSLATE_SPEED_MULTIPLIER
    } else {
        progress
    }
}
