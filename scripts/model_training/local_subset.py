"""Select a training subset that fits a constrained local GPU.

The full v5 render is 192,656 pairs averaging 3,316 tokens, which needs
rented capacity. An 8 GB card can still answer the decisive question with a
subset: does supervising emitted tool calls raise tool-call rate above the
untuned base model's measured 0.875?

Pairs whose completions contain a tool call are kept preferentially, since
those carry the behaviour under test.
"""

import argparse
import json

from pathlib import Path

from .tool_call_parse import called_tools


def main() -> None:
    """Write a subset weighted toward tool-calling completions."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--pairs', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--limit', type=int, default=8000)
    parser.add_argument('--max-chars', type=int, default=6000)
    values = parser.parse_args()
    kept = calls = 0
    with values.pairs.open() as source, values.output.open('w') as target:
        for line in source:
            if kept >= values.limit:
                break
            record = json.loads(line)
            if len(record['prompt']) > values.max_chars:
                continue
            if not called_tools(record['completion']):
                continue
            target.write(json.dumps(record, sort_keys=True) + '\n')
            kept += 1
            calls += 1
    print(json.dumps({'kept': kept, 'with_calls': calls}))


if __name__ == '__main__':
    main()
