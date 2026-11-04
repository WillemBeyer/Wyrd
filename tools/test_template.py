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
        (original/'crates/horizon-client/src/lib.rs').write_text('pub mod game;\n')
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
        (original/'crates/horizon-client/src/game.rs').write_text('''
pub mod native_hooks;
pub struct Game<'a> { pub positions: &'a mut Vec<[i32;3]>, pub messages: &'a mut Vec<String> }
impl Game<'_> {
    pub fn break_block(&mut self,pos:[i32;3])->i32 {self.positions.push(pos); 7}
}
#[test] fn original_body_and_result_are_preserved() {
    let mut positions=Vec::new();
    let mut messages=Vec::new();
    let mut game=Game {positions:&mut positions, messages:&mut messages};
    assert_eq!(game.break_block([1,2,3]),7);
    assert_eq!(game.break_block([-1,64,8]),7);
    assert_eq!(positions,vec![[1,2,3],[-1,64,8]]);
}
#[test] fn mod_registration_and_diagnostic_are_reachable() {
    let mut positions=Vec::new();
    let mut messages=Vec::new();
    let mut game=Game {positions:&mut positions, messages:&mut messages};
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
        assert report['injections'] == 3 and report['modules'] == 1
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
