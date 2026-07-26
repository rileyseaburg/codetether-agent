"""Measure empty-response rate for the served local model."""

import argparse
import json

from pathlib import Path

from .bench_cases import CODE_CASES
from .bench_client import chat


def main() -> None:
    """Repeat each code prompt and record empty completions."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--base-url', default='http://127.0.0.1:8099')
    parser.add_argument('--model', default='codetether-local')
    parser.add_argument('--repeats', type=int, default=3)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    results: list[dict[str, object]] = []
    for prompt, _ in CODE_CASES:
        for attempt in range(values.repeats):
            message = chat(
                values.base_url,
                values.model,
                [{'role': 'user', 'content': prompt}],
                max_tokens=120,
            )
            text = str(message.get('content') or '').strip()
            results.append(
                {
                    'attempt': attempt,
                    'chars': len(text),
                    'empty': not text,
                    'prompt': prompt[:60],
                }
            )
    empty = sum(1 for r in results if r['empty'])
    summary = {
        'empty_rate': empty / len(results),
        'samples': len(results),
        'results': results,
    }
    values.output.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + '\n'
    )
    print(
        json.dumps(
            {'empty_rate': summary['empty_rate'], 'samples': summary['samples']}
        )
    )


if __name__ == '__main__':
    main()
