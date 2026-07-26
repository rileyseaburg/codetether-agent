"""Build the full multi-source conversation corpus from local stores."""

import argparse
import json

from pathlib import Path

from .ingest_writer import ingest


def main() -> None:
    """Ingest CodeTether sessions and Codex rollouts into one JSONL corpus."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--index', type=Path, required=True)
    parser.add_argument('--codex', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--min-assistant', type=int, default=1)
    values = parser.parse_args()
    values.output.parent.mkdir(parents=True, exist_ok=True)
    report = ingest(
        _stores(values.index),
        values.codex,
        values.output,
        values.min_assistant,
    )
    manifest = values.output.parent / 'ingest-manifest.json'
    manifest.write_text(json.dumps(report, indent=2, sort_keys=True) + '\n')
    print(json.dumps(report, sort_keys=True))


def _stores(index: Path) -> list[Path]:
    """Return every indexed transcript directory."""
    data = json.loads(index.read_text())
    paths = {Path(str(store['path'])) for store in data.get('largest', [])}
    full = index.parent / 'host-stores.json'
    if full.exists():
        paths.update(Path(p) for p in json.loads(full.read_text()))
    return sorted(paths)


if __name__ == '__main__':
    main()
