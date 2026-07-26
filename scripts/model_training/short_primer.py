"""Append short instruction primers to a rendered pair dataset."""

import json

from pathlib import Path

from .render_pair import render
from .short_cases import messages


def append(target: Path, tokenizer: object, repeats: int) -> dict[str, object]:
    """Repeat short single-turn pairs so brief prompts stay answerable."""
    written = 0
    with target.open('ab') as output:
        for _ in range(repeats):
            for conversation in messages():
                for record in render(conversation, tokenizer, False):
                    record['metadata'] = {'source': 'short-primer'}
                    line = json.dumps(record, sort_keys=True) + '\n'
                    output.write(line.encode())
                    written += 1
    return {'records': written, 'repeats': repeats}
