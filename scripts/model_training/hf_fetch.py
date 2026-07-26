"""Download the v4 training splits from HuggingFace."""

import argparse
import json

from pathlib import Path

from huggingface_hub import hf_hub_download

from .hf_token import resolve_token


FILES = (
    'train-pairs.jsonl',
    'validation-pairs.jsonl',
    'render-v3-manifest.json',
)


def main() -> None:
    """Fetch each dataset file into the local bundle directory."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo', required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    values.output.mkdir(parents=True, exist_ok=True)
    token = resolve_token()
    fetched: list[dict[str, object]] = []
    for name in FILES:
        path = hf_hub_download(
            repo_id=values.repo,
            filename=name,
            repo_type='dataset',
            local_dir=str(values.output),
            token=token,
        )
        fetched.append({'file': name, 'bytes': Path(path).stat().st_size})
    print(json.dumps({'repo': values.repo, 'fetched': fetched}, sort_keys=True))


if __name__ == '__main__':
    main()
