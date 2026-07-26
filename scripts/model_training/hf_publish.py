"""Publish the v4 training bundle to a HuggingFace dataset repository."""

import argparse
import json

from pathlib import Path

from huggingface_hub import HfApi

from .hf_token import resolve_token


def main() -> None:
    """Upload dataset files so Colab can pull them without MinIO access."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo', required=True)
    parser.add_argument('--files', nargs='+', required=True)
    parser.add_argument('--private', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    api = HfApi(token=resolve_token())
    api.create_repo(
        repo_id=values.repo,
        repo_type='dataset',
        private=values.private,
        exist_ok=True,
    )
    uploaded: list[dict[str, object]] = []
    for name in values.files:
        path = Path(name)
        api.upload_file(
            path_or_fileobj=str(path),
            path_in_repo=path.name,
            repo_id=values.repo,
            repo_type='dataset',
        )
        uploaded.append({'file': path.name, 'bytes': path.stat().st_size})
    report = {
        'repo': values.repo,
        'repo_type': 'dataset',
        'private': values.private,
        'uploaded': uploaded,
    }
    values.output.write_text(
        json.dumps(report, indent=2, sort_keys=True) + '\n'
    )
    print(json.dumps(report, sort_keys=True))


if __name__ == '__main__':
    main()
