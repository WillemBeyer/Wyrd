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
pub const ORE_MEMORY_KEY: &str = "wyrd:ore_memory";
pub const ORE_BONUS_CHANCE: f32 = 0.10;
pub const DEEP_DELVER_KEY: &str = "wyrd:deep_delver";
pub const TUNNEL_COOLDOWN_GROUP: &str = "wyrd:tunnel_break";
pub const TUNNEL_COOLDOWN_TICKS: i32 = 600;
pub const TUNNEL_KEY_NAME: &str = "tunnel_break";
pub const TUNNEL_KEY_LABEL: &str = "Tunnel Break";
pub const TUNNEL_DEFAULT_KEY: &str = "key.keyboard.v";

pub const DEEPSLATE_EFFECT: &str = "+15% mining speed on deepslate";
pub const ORE_EFFECT: &str = "10% chance for +1 ore drop (no Silk Touch)";

pub fn is_eligible_pickaxe(item: &str) -> bool {
    matches!(
        item,
        "minecraft:iron_pickaxe" | "minecraft:diamond_pickaxe" | "minecraft:netherite_pickaxe"
    )
}

mod boost_flags {
    #![allow(clippy::missing_const_for_thread_local)]

    thread_local! {
        pub static DEEPSLATE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        pub static ORE_RESOURCE: std::cell::Cell<Option<([i32; 3], &'static str)>> =
            const { std::cell::Cell::new(None) };
    }
}

pub fn set_deepslate_boost(active: bool) {
    boost_flags::DEEPSLATE.set(active);
}

pub fn take_deepslate_boost() -> bool {
    boost_flags::DEEPSLATE.replace(false)
}

pub fn set_ore_bonus_resource(pos: [i32; 3], resource: Option<&'static str>) {
    boost_flags::ORE_RESOURCE.set(resource.map(|resource| (pos, resource)));
}

pub fn take_ore_bonus_resource(pos: [i32; 3], item: &str) -> Option<&'static str> {
    let (expected_pos, resource) = boost_flags::ORE_RESOURCE.get()?;
    if expected_pos != pos || resource != item {
        return None;
    }
    boost_flags::ORE_RESOURCE.set(None);
    Some(resource)
}

pub fn ore_resource(block: &str) -> Option<&'static str> {
    let ore = block.strip_prefix("minecraft:")?;
    let ore = ore.strip_prefix("deepslate_").unwrap_or(ore);
    Some(match ore {
        "coal_ore" => "minecraft:coal",
        "copper_ore" => "minecraft:raw_copper",
        "iron_ore" => "minecraft:raw_iron",
        "gold_ore" => "minecraft:raw_gold",
        "redstone_ore" => "minecraft:redstone",
        "lapis_ore" => "minecraft:lapis_lazuli",
        "diamond_ore" => "minecraft:diamond",
        "emerald_ore" => "minecraft:emerald",
        "nether_gold_ore" => "minecraft:gold_nugget",
        "nether_quartz_ore" => "minecraft:quartz",
        _ => return None,
    })
}

pub fn is_tunnel_stone(block: &str) -> bool {
    matches!(
        block,
        "minecraft:stone"
            | "minecraft:deepslate"
            | "minecraft:granite"
            | "minecraft:diorite"
            | "minecraft:andesite"
            | "minecraft:tuff"
            | "minecraft:calcite"
            | "minecraft:netherrack"
            | "minecraft:blackstone"
            | "minecraft:basalt"
            | "minecraft:smooth_basalt"
            | "minecraft:end_stone"
    )
}

pub fn tunnel_plan(origin: [i32; 3], look: [f64; 3]) -> Vec<Vec<[i32; 3]>> {
    let [x, y, z] = look.map(f64::abs);
    let direction = if y >= x && y >= z {
        [0, if look[1] >= 0.0 { 1 } else { -1 }, 0]
    } else if x >= z {
        [if look[0] >= 0.0 { 1 } else { -1 }, 0, 0]
    } else {
        [0, 0, if look[2] >= 0.0 { 1 } else { -1 }]
    };
    (0..6)
        .map(|step| {
            (-1..=1)
                .flat_map(|a| {
                    (-1..=1).map(move |b| {
                        if direction[1] != 0 {
                            [
                                origin[0] + a,
                                origin[1] + direction[1] * step,
                                origin[2] + b,
                            ]
                        } else if direction[0] != 0 {
                            [
                                origin[0] + direction[0] * step,
                                origin[1] + a,
                                origin[2] + b,
                            ]
                        } else {
                            [
                                origin[0] + a,
                                origin[1] + b,
                                origin[2] + direction[2] * step,
                            ]
                        }
                    })
                })
                .collect()
        })
        .collect()
}

pub struct PendingTunnel {
    pub pos: [i32; 3],
    pub look: [f64; 3],
    pub target: String,
}

mod tunnel_state {
    #![allow(clippy::missing_const_for_thread_local)]

    thread_local! {
        pub static ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        pub static PENDING: std::cell::RefCell<Option<super::PendingTunnel>> =
            const { std::cell::RefCell::new(None) };
        pub static WEAR_ATTEMPTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
}

pub fn set_tunnel_armed(armed: bool) {
    tunnel_state::ARMED.set(armed);
    clear_pending_tunnel();
}

pub fn tunnel_armed() -> bool {
    tunnel_state::ARMED.get()
}

pub fn take_tunnel_armed() -> bool {
    tunnel_state::ARMED.replace(false)
}

pub fn set_pending_tunnel(pos: [i32; 3], look: [f64; 3], target: String) {
    tunnel_state::PENDING.with(|pending| {
        *pending.borrow_mut() = Some(PendingTunnel { pos, look, target });
    });
}

pub fn take_pending_tunnel() -> Option<PendingTunnel> {
    tunnel_state::PENDING.with(|pending| pending.take())
}

pub fn clear_pending_tunnel() {
    tunnel_state::PENDING.with(|pending| {
        let _ = pending.take();
    });
}

pub fn mark_wear_attempt() {
    tunnel_state::WEAR_ATTEMPTED.set(true);
}

pub fn clear_wear_attempt() {
    tunnel_state::WEAR_ATTEMPTED.set(false);
}

pub fn take_wear_attempt() -> bool {
    tunnel_state::WEAR_ATTEMPTED.replace(false)
}

pub fn tunnel_cooldown_seconds(remaining: f32) -> u32 {
    (remaining * (TUNNEL_COOLDOWN_TICKS / 20) as f32).ceil() as u32
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
    fn eligible_pickaxe_tiers_share_the_deep_delver_rule() {
        for item in [
            "minecraft:iron_pickaxe",
            "minecraft:diamond_pickaxe",
            "minecraft:netherite_pickaxe",
        ] {
            assert!(is_eligible_pickaxe(item), "{item}");
        }
        for item in [
            "minecraft:stone_pickaxe",
            "minecraft:golden_pickaxe",
            "minecraft:wooden_pickaxe",
            "minecraft:iron_axe",
            "minecraft:air",
        ] {
            assert!(!is_eligible_pickaxe(item), "{item}");
        }
    }

    #[test]
    fn deepslate_boost_is_consumed_once() {
        set_deepslate_boost(true);
        assert!(take_deepslate_boost());
        assert!(!take_deepslate_boost());
    }

    #[test]
    fn ore_bonus_resource_is_consumed_once() {
        set_ore_bonus_resource([0; 3], Some("minecraft:raw_iron"));
        assert_eq!(
            take_ore_bonus_resource([0; 3], "minecraft:raw_iron"),
            Some("minecraft:raw_iron")
        );
        assert_eq!(take_ore_bonus_resource([0; 3], "minecraft:raw_iron"), None);
    }

    #[test]
    fn tunnel_plan_follows_the_look_axis() {
        let forward = tunnel_plan([10, 64, -5], [1.0, 0.0, 0.0]);
        assert_eq!(forward.len(), 6);
        assert!(forward.iter().all(|slice| slice.len() == 9));
        assert!(forward[0].contains(&[10, 64, -5]));
        assert!(forward[5].contains(&[15, 64, -5]));

        let up = tunnel_plan([10, 64, -5], [0.0, 1.0, 0.0]);
        assert!(up[0].contains(&[11, 64, -4]));
        assert!(up[5].contains(&[10, 69, -5]));
    }

    #[test]
    fn tunnel_plan_uses_the_dominant_look_axis() {
        let plan = tunnel_plan([0, 0, 0], [0.2, 0.9, 0.1]);
        assert!(plan[1].contains(&[0, 1, 0]));
    }

    #[test]
    fn tunnel_stone_list_is_explicit() {
        for block in [
            "minecraft:stone",
            "minecraft:deepslate",
            "minecraft:tuff",
            "minecraft:netherrack",
            "minecraft:end_stone",
        ] {
            assert!(is_tunnel_stone(block), "{block}");
        }
        assert!(!is_tunnel_stone("minecraft:dirt"));
        assert!(!is_tunnel_stone("minecraft:iron_ore"));
    }

    #[test]
    fn tunnel_state_arms_and_clears_pending() {
        set_tunnel_armed(true);
        assert!(tunnel_armed());
        set_pending_tunnel([1, 2, 3], [0.0, 1.0, 0.0], "minecraft:stone".to_owned());
        assert!(take_pending_tunnel().is_some());
        assert!(take_tunnel_armed());
        assert!(!tunnel_armed());
        assert!(take_pending_tunnel().is_none());
    }

    #[test]
    fn wear_attempt_is_consumed_once() {
        clear_wear_attempt();
        assert!(!take_wear_attempt());
        mark_wear_attempt();
        assert!(take_wear_attempt());
        assert!(!take_wear_attempt());
    }

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
