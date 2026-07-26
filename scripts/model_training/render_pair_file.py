"""Write prompt/completion JSONL with bounded token budgets."""

import hashlib
import json

from pathlib import Path

from .render_pair import render


def build(
    source: Path, target: Path, tokenizer: object, limit: int
) -> dict[str, object]:
    """Render each assistant turn as a masked supervision record."""
    digest = hashlib.sha256()
    included = excluded = total = 0
    with_tools = 0
    with target.open('wb') as output:
        for index, raw in enumerate(source.read_text().splitlines()):
            value = json.loads(raw)
            use_tools = index % 2 == 0
            for record in render(value['messages'], tokenizer, use_tools):
                tokens = _count(tokenizer, record)
                if tokens > limit:
                    excluded += 1
                    continue
                payload = dict(record)
                payload['metadata'] = value['metadata']
                line = json.dumps(payload, sort_keys=True) + '\n'
                encoded = line.encode()
                output.write(encoded)
                digest.update(encoded)
                included += 1
                total += tokens
                with_tools += int(use_tools)
    return {
        'source': str(source),
        'path': str(target),
        'included': included,
        'excluded_over_limit': excluded,
        'tool_schema_records': with_tools,
        'tokens': total,
        'mean_tokens': round(total / included, 2) if included else 0,
        'bytes': target.stat().st_size,
        'sha256': digest.hexdigest(),
    }


def _count(tokenizer: object, record: dict[str, str]) -> int:
    text = record['prompt'] + record['completion']
    return len(tokenizer(text, add_special_tokens=False)['input_ids'])
