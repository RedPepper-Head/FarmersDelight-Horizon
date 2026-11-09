#!/usr/bin/env python3
"""Package a pinned Horizon native mixin bundle (stdlib only)."""
import argparse
import hashlib
import json
import re
import sys
import tomllib
import zipfile
from pathlib import Path

MOD_ID = "farmers_delight_horizon"
VERSION = "0.1.0"
GAME = "26.3"
HORIZON = "fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca"
HEX40 = re.compile(r"[0-9a-f]{40}\Z")
FILES = (
    f"mixins/{MOD_ID}/horizon-mixin.toml",
    f"mixins/{MOD_ID}/break.rs",
    f"mixins/{MOD_ID}/runtime.rs",
)


def package(repo: Path, output: Path, mixins: str) -> None:
    if not HEX40.fullmatch(mixins):
        raise ValueError("--mixins must be a full lowercase 40-character Git commit SHA")
    toml = (repo / FILES[0]).read_text(encoding="utf-8")
    parsed = tomllib.loads(toml)
    if parsed.get("mod", {}).get("id") != MOD_ID:
        raise ValueError("Unexpected mixin mod id")
    if parsed["mod"].get("version") != VERSION:
        raise ValueError("Unexpected mixin version")
    if parsed.get("compatibility", {}).get("horizon_revision") != HORIZON:
        raise ValueError("Unexpected Horizon revision")
    for rel in FILES:
        if not (repo / rel).is_file():
            raise FileNotFoundError(repo / rel)
    manifest = {
        "schema": 1,
        "id": MOD_ID,
        "version": VERSION,
        "game": GAME,
        "horizon": HORIZON,
        "mixins": mixins,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    contents = {
        "horizon-native.json": (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode("utf-8")
    }
    contents.update({rel: (repo / rel).read_bytes() for rel in FILES})
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for rel, data in sorted(contents.items()):
            entry = zipfile.ZipInfo(rel, (2026, 1, 1, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.external_attr = 0o100644 << 16
            archive.writestr(entry, data)
    with zipfile.ZipFile(output) as archive:
        if sorted(archive.namelist()) != sorted(("horizon-native.json", *FILES)):
            raise ValueError("Unexpected package entries")
    digest = hashlib.sha256(output.read_bytes()).hexdigest()
    print(f"Created {output} (sha256: {digest}; not tested against Horizon)")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mixins", required=True, help="40-character horizon-mixins commit SHA (verify against the target launcher before use)")
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--output", type=Path, default=Path("dist/farmers_delight_horizon-0.1.0.hnmod"))
    args = parser.parse_args()
    try:
        package(args.repo, args.output, args.mixins)
    except (OSError, ValueError) as error:
        print(f"Packaging failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
