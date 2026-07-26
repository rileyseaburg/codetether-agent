"""Accumulate ingest counters for corpus evidence."""

import collections

from dataclasses import dataclass, field


@dataclass
class Stats:
    """Mutable counters describing one ingest run."""

    conversations: int = 0
    messages: int = 0
    assistant_turns: int = 0
    tool_calls: int = 0
    duplicates: int = 0
    skipped: int = 0
    sources: collections.Counter[str] = field(
        default_factory=collections.Counter
    )

    def add(self, messages: list[dict[str, object]], source: str) -> None:
        """Record one accepted conversation."""
        self.conversations += 1
        self.messages += len(messages)
        self.sources[source] += 1
        for message in messages:
            if message.get('role') != 'assistant':
                continue
            self.assistant_turns += 1
            calls = message.get('tool_calls')
            if isinstance(calls, list):
                self.tool_calls += len(calls)

    def report(self, path: str) -> dict[str, object]:
        """Return a JSON-safe summary."""
        return {
            'path': path,
            'conversations': self.conversations,
            'messages': self.messages,
            'assistant_turns': self.assistant_turns,
            'tool_calls': self.tool_calls,
            'duplicates_skipped': self.duplicates,
            'low_signal_skipped': self.skipped,
            'sources': dict(self.sources),
        }
