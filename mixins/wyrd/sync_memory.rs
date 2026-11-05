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

// The integrated server sends inventory through this encoder. Preserve Wyrd
// memory markers and lore so the client can render the pickaxe's tooltip.
fn sync_pickaxe_memory(mut network: Option<Stack>, local: &Option<ItemStack>) -> Option<Stack> {
    use horizon_protocol::item::{Component, component};
    use horizon_world::nbt::Tag;

    let (Some(network_stack), Some(local_stack)) = (&mut network, local) else {
        return network;
    };
    // This helper runs in the library crate; Horizon's game module (and thus
    // `crate::game::wyrd`) lives in the binary crate, so mirror the runtime's
    // eligibility rule here instead of importing it.
    if !matches!(
        local_stack.item.as_ref(),
        "minecraft:iron_pickaxe" | "minecraft:diamond_pickaxe" | "minecraft:netherite_pickaxe"
    ) {
        return network;
    }
    let Some(data) = local_stack.data() else {
        return network;
    };
    let Some(custom_data @ Tag::Compound(_)) = data.other("minecraft:custom_data") else {
        return network;
    };

    for (id, component) in [
        (
            component::CUSTOM_DATA,
            Some(Component::Nbt(custom_data.clone())),
        ),
        (
            component::LORE,
            match data.other("minecraft:lore") {
                Some(Tag::List(lines)) => Some(Component::Lore(lines.clone())),
                _ => None,
            },
        ),
    ] {
        if let Some(component) = component
            && !network_stack.added.iter().any(|(kind, _)| *kind == id)
        {
            network_stack.added.push((id, component));
        }
    }
    network
}
