#!/usr/bin/env python3
"""Check reproducibility, source contents and compatibility guards without Rust."""
import contextlib
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
import zipfile

import package as packager


class PackageTests(unittest.TestCase):
    def setUp(self):
        self.previous_root = packager.ROOT
        self.scratch = tempfile.TemporaryDirectory(prefix='wyrd-package-')
        self.root = Path(self.scratch.name)
        shutil.copy2(self.previous_root / 'compatibility.json', self.root)
        shutil.copytree(self.previous_root / 'mixins', self.root / 'mixins')
        packager.ROOT = self.root

    def tearDown(self):
        packager.ROOT = self.previous_root
        self.scratch.cleanup()

    def build(self):
        with contextlib.redirect_stdout(io.StringIO()):
            packager.package()
        return self.root / 'target/wyrd-0.1.0.hnmod'

    def test_reproducible_source_bundle_has_wyrd_identity_and_notices(self):
        path = self.build()
        first = path.read_bytes()
        self.assertEqual(first, self.build().read_bytes())
        with zipfile.ZipFile(path) as archive:
            self.assertEqual(set(archive.namelist()), {
                'horizon-native.json', 'mixins/wyrd/horizon-mixin.toml',
                'mixins/wyrd/break.rs', 'mixins/wyrd/runtime.rs',
                'mixins/wyrd/list.rs', 'mixins/wyrd/command.rs',
                'mixins/wyrd/memory.rs', 'mixins/wyrd/mining_bonus.rs',
                'mixins/wyrd/iron_bonus.rs', 'mixins/wyrd/effects_tooltip.rs',
                'mixins/wyrd/sync_memory.rs',
            })
            manifest = json.loads(archive.read('horizon-native.json'))
            self.assertEqual(manifest['id'], 'wyrd')
            self.assertEqual(manifest['version'], '0.1.0')
            for name in archive.namelist():
                if name != 'horizon-native.json':
                    self.assertIn(b'MIT License', archive.read(name))
                    self.assertIn(b'Horizon Veinminer contributors', archive.read(name))

    def test_mismatched_horizon_pin_does_not_emit_bundle(self):
        path = self.root / 'compatibility.json'
        compat = json.loads(path.read_text())
        compat['horizon_revision'] = '0' * 40
        path.write_text(json.dumps(compat))
        with self.assertRaisesRegex(SystemExit, 'pin different Horizon revisions'):
            self.build()
        self.assertFalse((self.root / 'target').exists())


if __name__ == '__main__':
    unittest.main()
