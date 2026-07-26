"""Deduplicating sink for ingested conversations."""

import hashlib
import json

from dataclasses import dataclass, field
from pathlib import Path
from typing import BinaryIO

from .ingest_stats import Stats


@dataclass
class Sink:
    """Write novel conversations and track corpus statistics."""

    handle: BinaryIO
    stats: Stats
    min_assistant: int
    seen: set[str] = field(default_factory=set)

    def emit(
        self, messages: list[dict[str, object]], path: Path, source: str
    ) -> None:
        """Write one conversation when novel and carrying supervision."""
        assistants = [m for m in messages if m.get('role') == 'assistant']
        if len(assistants) < self.min_assistant:
            self.stats.skipped += 1
            return
        body = json.dumps(messages, sort_keys=True)
        digest = hashlib.sha256(body.encode()).hexdigest()
        if digest in self.seen:
            self.stats.duplicates += 1
            return
        self.seen.add(digest)
        record = {
            'messages': messages,
            'metadata': {
                'source': source,
                'path': str(path),
                'sha256': digest,
            },
        }
        self.handle.write((json.dumps(record, sort_keys=True) + '\n').encode())
        self.stats.add(messages, source)
