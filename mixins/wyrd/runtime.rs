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

// Put testable mod logic here. This file is both the standalone Cargo library
// and the injected `crate::game::wyrd` module; no generated copy exists.

pub const DEEPSLATE_MEMORY_KEY: &str = "wyrd:deepslate_memory";
pub const DEEPSLATE_SPEED_MULTIPLIER: f32 = 1.15;
pub const IRON_MEMORY_KEY: &str = "wyrd:iron_memory";
pub const IRON_BONUS_CHANCE: f32 = 0.20;

pub const DEEPSLATE_EFFECT: &str = "+15% mining speed on deepslate";
pub const IRON_EFFECT: &str = "20% chance for +1 raw iron from iron ore (no Silk Touch)";

pub fn is_iron_ore(block: &str) -> bool {
    matches!(block, "minecraft:iron_ore" | "minecraft:deepslate_iron_ore")
}

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[wyrd] Breaking block at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    log::info!(target: "horizon::wyrd", "{}", block_break_message(pos));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[wyrd] Breaking block at -3, 64, 12"
        );
    }

    #[test]
    fn break_attempt_reaches_the_info_logger() {
        use std::sync::Mutex;

        static MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
        struct Capture;
        impl log::Log for Capture {
            fn enabled(&self, _: &log::Metadata<'_>) -> bool {
                true
            }
            fn log(&self, record: &log::Record<'_>) {
                MESSAGES
                    .lock()
                    .unwrap()
                    .push(format!("{} {}", record.level(), record.args()));
            }
            fn flush(&self) {}
        }
        static LOGGER: Capture = Capture;
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Info);

        before_break([-3, 64, 12]);

        assert!(
            MESSAGES
                .lock()
                .unwrap()
                .iter()
                .any(|message| { message == "INFO [wyrd] Breaking block at -3, 64, 12" })
        );
    }
}
