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

fn tunnel_keybind(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::wyrd::{TUNNEL_DEFAULT_KEY, TUNNEL_KEY_NAME};

    if !pressed
        || !game.native_on("wyrd")
        || !game.native_playing()
        || !game.native_window()
        || !game.has_singleplayer_server()
    {
        return;
    }
    if key
        != game
            .native_mod_key("wyrd", TUNNEL_KEY_NAME, TUNNEL_DEFAULT_KEY)
            .as_str()
    {
        return;
    }
    if game.remote.is_some() {
        use horizon_protocol::packets::play::{self, Serverbound};
        game.send(Serverbound::ChatCommand(play::ChatCommand {
            command: "wyrd tunnel".to_owned(),
        }));
    } else {
        game.run_command("wyrd tunnel");
    }
}
