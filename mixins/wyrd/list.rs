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

// Native code bundles do not automatically add their own Mods menu row.
fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::wyrd::{TUNNEL_DEFAULT_KEY, TUNNEL_KEY_LABEL, TUNNEL_KEY_NAME};

    mods.push(crate::game::native_hooks::NativeMod {
        id: "wyrd".into(),
        title: "Wyrd 0.1.0".into(),
        description:
            "Bind a pickaxe at an enchanting table, then press the Tunnel Break key while playing."
                .into(),
        keys: vec![[
            TUNNEL_KEY_NAME.into(),
            TUNNEL_KEY_LABEL.into(),
            TUNNEL_DEFAULT_KEY.into(),
        ]],
    });
    log::info!(target: "horizon::wyrd", "[wyrd] Native scaffold loaded (0.1.0)");
}
