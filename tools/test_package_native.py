"""Tests for the native package format; no Horizon build is attempted."""
import json
import tempfile
import unittest
import zipfile
from pathlib import Path

from package_native import FILES, GAME, HORIZON, MOD_ID, VERSION, package


class NativePackageTests(unittest.TestCase):
    def setUp(self):
        self.repo = Path(__file__).resolve().parents[1]

    def test_archive_structure_and_manifest(self):
        # Synthetic SHA for testing serialization only, NOT a compatible revision.
        synthetic_sha = "0" * 40
        with tempfile.TemporaryDirectory() as directory:
            destination = Path(directory) / "smoke.hnmod"
            package(self.repo, destination, synthetic_sha)
            with zipfile.ZipFile(destination) as archive:
                self.assertEqual(
                    sorted(archive.namelist()),
                    sorted(["horizon-native.json", *FILES]),
                )
                manifest = json.loads(archive.read("horizon-native.json"))
                self.assertEqual(
                    manifest,
                    {
                        "schema": 1,
                        "id": MOD_ID,
                        "version": VERSION,
                        "game": GAME,
                        "horizon": HORIZON,
                        "mixins": synthetic_sha,
                    },
                )
                for path in FILES:
                    self.assertEqual(archive.read(path), (self.repo / path).read_bytes())

    def test_rejects_invalid_revision(self):
        with tempfile.TemporaryDirectory() as directory:
            destination = Path(directory) / "invalid.hnmod"
            with self.assertRaises(ValueError):
                package(self.repo, destination, "not-a-commit")
            self.assertFalse(destination.exists())


if __name__ == "__main__":
    unittest.main()
