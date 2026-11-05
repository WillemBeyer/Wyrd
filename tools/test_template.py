#!/usr/bin/env python3
"""Compile the real bundle on a tiny source fixture; no Horizon/assets required."""
import json
from pathlib import Path
import subprocess
import tempfile
import tomllib
import horizon


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(result.stdout + result.stderr)
    return result


def snapshot(root):
    return {str(p.relative_to(root)): (horizon.digest(p.read_bytes()), p.stat().st_mtime_ns)
            for p in root.rglob('*') if p.is_file()}


def audit(original, stage, plan):
    run(['cargo', 'run', '--quiet', '--locked', '--offline', '-p', 'native-mod-source-audit',
         '--', str(original), str(stage), str(plan)], cwd=horizon.ROOT)


def main():
    root = horizon.ROOT
    (root/'target').mkdir(exist_ok=True)
    run(['cargo', 'build', '--locked', '--offline', '-p', 'horizon-mixins-cli'], cwd=horizon.MIXINS)
    cli = horizon.MIXINS/'target/debug/horizon-mixins'
    manifests = list((root/'mixins').glob('*/horizon-mixin.toml'))
    assert len(manifests) == 1, 'this template test expects one example bundle'
    manifest = tomllib.loads(manifests[0].read_text())
    with tempfile.TemporaryDirectory(prefix='template-test-', dir=root/'target') as scratch:
        scratch = Path(scratch)
        original, stage = scratch/'original', scratch/'stage'
        (original/'crates/horizon-client/src').mkdir(parents=True)
        (original/'Cargo.toml').write_text('[workspace]\nmembers=["crates/horizon-client"]\nresolver="2"\n')
        (original/'crates/horizon-client/Cargo.toml').write_text('[package]\nname="horizon-client"\nversion="0.1.0"\nedition="2024"\n[dependencies]\nlog="0.4"\n')
        (original/'crates/horizon-client/src/lib.rs').write_text('''
extern crate self as horizon_world;
extern crate self as horizon_protocol;
extern crate self as horizon_client;
pub mod text {
    pub struct Text(pub String);
    impl Text {
        pub fn literal(text: &str) -> Self { Self(text.into()) }
    }
}
pub mod packets {
    pub mod play {
        pub struct Text { pub text: crate::nbt::Tag }
        pub enum Clientbound { SetActionBarText(Text) }
    }
}
pub mod game;
pub mod nbt {
    use std::collections::BTreeMap;
    pub type JavaString = String;
    #[derive(Clone, Debug, PartialEq)]
    pub enum Tag {
        Byte(i8), String(String), List(Vec<Tag>), Compound(BTreeMap<String, Tag>),
    }
}
''')
        (original/'crates/horizon-client/src/game').mkdir()
        (original/'crates/horizon-client/src/game/native_hooks.rs').write_text('''
use super::Game;
#[derive(Default)]
pub struct NativeMod {
    pub id: String, pub title: String, pub description: String, pub keys: Vec<[String; 3]>,
}
impl Game<'_> {
    pub fn native_mods(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod { id: "host".into(), ..NativeMod::default() });
    }
    pub fn native_command(&mut self, command: &str, ran: &mut bool) { let _ = (command, ran); }
    pub fn native_on(&self, _: &str) -> bool { true }
    pub fn native_chat(&mut self, text: &str) { self.messages.push(text.to_owned()); }
}
''')
        (original/'crates/horizon-client/src/game/hand.rs').write_text('''
use super::Game;
impl Game<'_> {
    fn destroy_progress(&self, pos: [i32; 3]) -> f32 {
        if pos[1] < 0 { return 0.0; }
        0.1
    }
}
''')
        (original/'crates/horizon-client/src/game/stations.rs').write_text('''
use super::{Game, ItemStack};
impl Game<'_> {
    fn draw_stack_tooltip_with(&mut self, stack: &ItemStack, creative: bool) {
        let mut lines = self.stack_tooltip_lines(stack, false, creative);
        lines.clear();
    }
    fn stack_tooltip_lines(
        &self, stack: &ItemStack, nested: bool, creative: bool,
    ) -> Vec<(crate::text::Text, u32)> {
        let _ = (stack, nested, creative);
        Vec::new()
    }
}
''')
        (original/'crates/horizon-client/src/game.rs').write_text('''
pub mod native_hooks;
pub mod hand;
pub mod stations;
use crate::nbt::Tag;
#[derive(Default)]
struct World { random: Random }
#[derive(Default)]
struct Random;
impl Random {
    fn next_float(&mut self) -> f32 { 1.0 }
}
impl World {
    fn block(&self, _: i32, _: i32, _: i32) -> u32 { 0 }
}
#[derive(Default)]
struct Resources;
struct Facts { name: String }
impl Resources {
    fn facts(&self) -> &Self { self }
    fn get(&self, _: u32) -> Facts { Facts { name: "minecraft:deepslate".into() } }
}
#[derive(Default)]
struct Titles;
impl Titles {
    fn set_overlay(&mut self, _: crate::text::Text, _: bool) {}
}
use std::{collections::BTreeMap, sync::Arc};
#[derive(Default)]
struct Data(BTreeMap<String, Tag>);
impl Data {
    fn other(&self, key: &str) -> Option<&Tag> { self.0.get(key) }
    fn set_other(&mut self, key: &str, value: Option<Tag>) {
        self.0.insert(key.into(), value.unwrap());
    }
}
type BlockStateId = u32;
type ItemStack = Stack;
struct Stack { item: Arc<str>, data: Data }
impl Stack {
    fn new(item: &str, _: u32) -> Self {
        Self { item: item.into(), data: Data::default() }
    }
    fn enchantment_level(&self, _: &str) -> u32 { 0 }
    fn data_mut(&mut self) -> &mut Data { &mut self.data }
    fn data(&self) -> Option<&Data> { Some(&self.data) }
}
#[derive(Default)]
struct Rules;
impl Rules {
    fn bool(&self, _: &str) -> bool { true }
}
#[derive(Default)]
struct Ui { rules: Rules }
pub struct Game<'a> {
    pub positions: &'a mut Vec<[i32;3]>, pub messages: &'a mut Vec<String>,
    inventory: [Option<Stack>; 2], selected: usize, wear_calls: usize,
    ui: Ui, world: World, res: Resources, titles: Titles, lan_acting: Option<u32>,
    guest_packets: Vec<crate::packets::play::Clientbound>,
}
impl Game<'_> {
    fn held(&self) -> Option<&Stack> { self.inventory[self.selected].as_ref() }
    pub fn break_block(&mut self,pos:[i32;3])->i32 {self.positions.push(pos); 7}
    fn pop_resource(&mut self, _: [i32; 3], _: ItemStack) {}
    fn block_loot_at(
        &mut self, pos: [i32; 3], state: BlockStateId, held: Option<&ItemStack>,
    ) -> Vec<ItemStack> {
        let _ = (pos, state, held);
        Vec::new()
    }
    fn mine_block_with_held(&mut self, name: &str, hardness: f32) {
        let _ = (name, hardness);
        self.wear_calls += 1;
    }
}
#[test] fn memory_is_specific_to_the_used_pickaxe_and_preserves_components() {
    let mut positions = Vec::new();
    let mut messages = Vec::new();
    let pickaxe = || Some(Stack::new("minecraft:iron_pickaxe", 1));
    let mut game = Game { positions: &mut positions, messages: &mut messages,
        inventory: [pickaxe(), pickaxe()], selected: 1, wear_calls: 0, ui: Ui::default(), world: World::default(), res: Resources, titles: Titles, lan_acting: None, guest_packets: Vec::new() };
    let unrelated = Tag::Byte(9);
    let original_lore = Tag::String("Existing line".into());
    let data = game.inventory[1].as_mut().unwrap().data_mut();
    data.set_other("minecraft:custom_data", Some(Tag::Compound(BTreeMap::from([
        ("unrelated".into(), unrelated.clone()),
    ]))));
    data.set_other("minecraft:lore", Some(Tag::List(vec![original_lore.clone()])));
    game.mine_block_with_held("minecraft:stone", 1.0);
    assert_eq!(game.inventory[1].as_ref().unwrap().data.other("minecraft:lore"),
        Some(&Tag::List(vec![original_lore.clone()])));
    game.mine_block_with_held("minecraft:deepslate", 3.0);
    game.mine_block_with_held("minecraft:deepslate", 3.0);
    assert_eq!(game.wear_calls, 3);
    assert!(game.inventory[0].as_ref().unwrap().data.0.is_empty());
    let data = &game.inventory[1].as_ref().unwrap().data;
    assert_eq!(data.other("minecraft:lore"),
        Some(&Tag::List(vec![original_lore, Tag::String("Deepslate Memory".into())])));
    let Some(Tag::Compound(custom)) = data.other("minecraft:custom_data") else { panic!() };
    assert_eq!(custom.get("unrelated"), Some(&unrelated));
    assert_eq!(custom.get("wyrd:deepslate_memory"), Some(&Tag::Byte(1)));
}
#[test] fn original_body_and_result_are_preserved() {
    let mut positions=Vec::new();
    let mut messages=Vec::new();
    let mut game=Game {positions:&mut positions, messages:&mut messages, inventory:[None, None], selected:0, wear_calls:0, ui: Ui::default(), world: World::default(), res: Resources, titles: Titles, lan_acting: None, guest_packets: Vec::new()};
    assert_eq!(game.break_block([1,2,3]),7);
    assert_eq!(game.break_block([-1,64,8]),7);
    assert_eq!(positions,vec![[1,2,3],[-1,64,8]]);
}
#[test] fn mod_registration_and_diagnostic_are_reachable() {
    let mut positions=Vec::new();
    let mut messages=Vec::new();
    let mut game=Game {positions:&mut positions, messages:&mut messages, inventory:[None, None], selected:0, wear_calls:0, ui: Ui::default(), world: World::default(), res: Resources, titles: Titles, lan_acting: None, guest_packets: Vec::new()};
    let mut mods=Vec::new();
    game.native_mods(&mut mods);
    assert_eq!(mods.iter().filter(|m| m.id=="wyrd").count(),1);
    assert!(mods.iter().any(|m| m.id=="host"));
    let mut ran=false;
    game.native_command("wyrd", &mut ran);
    assert!(ran);
    assert!(game.messages[0].contains("loaded and enabled"));
    ran=false;
    game.native_command("unrelated", &mut ran);
    assert!(!ran);
    assert_eq!(game.messages.len(),1);
}
''')
        (original/'rust-toolchain.toml').write_bytes((root/'rust-toolchain.toml').read_bytes())
        (original/'.horizon-revision').write_text(horizon.PIN+'\n')
        run(['cargo', 'generate-lockfile', '--offline'], cwd=original)
        before = snapshot(original)
        base = ['--workspace', str(original), '--mods', str(root/'mixins')]
        report = json.loads(run([str(cli), 'check', *base, '--out', str(stage), '--trust-native-mods']).stdout)
        assert report['injections'] == 7 and report['modules'] == 1
        assert 'test result: ok.' in report['cargo_stdout']
        first = snapshot(stage)
        second = json.loads(run([str(cli), 'check', *base, '--out', str(stage), '--trust-native-mods']).stdout)
        assert not second['stage_changed'] and snapshot(stage) == first
        assert snapshot(original) == before
        plan_path = scratch/'plan.json'
        run([str(cli), 'plan', *base, '--out', str(plan_path)])
        plan = json.loads(plan_path.read_text())
        assert plan['schema'] == horizon.COMPAT['mixin_schema']
        assert all(i['mod_id'] == manifest['mod']['id'] for i in plan['injections'])
        audit(original, stage, plan_path)
        (original/'.horizon-revision').write_text('incompatible\n')
        bad = subprocess.run([str(cli), 'apply', *base, '--out', str(stage)], capture_output=True, text=True)
        assert bad.returncode and json.loads(bad.stderr)['code'] == 'E_REVISION'
        assert snapshot(stage) == first
        (original/'.horizon-revision').write_text(horizon.PIN+'\n')
        source = original/'crates/horizon-client/src/game.rs'
        source.write_text(source.read_text().replace('break_block', 'renamed_break_block'))
        bad = subprocess.run([str(cli), 'apply', *base, '--out', str(stage)], capture_output=True, text=True)
        assert bad.returncode and json.loads(bad.stderr)['code'] == 'E_TARGET_NOT_FOUND'
        assert snapshot(stage) == first
    print(json.dumps({'status': 'ok', 'compiled_runtime_and_original_body': True,
                      'only_planned_asts_changed': True, 'deterministic_bytes_and_mtimes': True,
                      'incompatible_revision_and_target_rejected': True}, indent=2))


if __name__ == '__main__':
    main()
