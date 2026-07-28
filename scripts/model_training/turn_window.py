"""Bound supervision context so rendering stays linear in corpus size.

Some conversations exceed 1,600 messages. Rendering every prefix is
quadratic, so each assistant turn keeps only a recent context window and
very long conversations may contribute a bounded number of turns.

Context windowing is what makes rendering linear; the per-conversation turn
cap is a separate throughput lever. Measured on 4,000 conversations holding
79,800 assistant turns:

| cap | turns kept |
|----:|-----------:|
|  24 |      25.2% |
|  64 |      39.2% |
| 128 |      49.9% |
| 256 |      64.0% |
| off |     100.0% |
"""

import os


CONTEXT_TURNS = 12
MAX_TURNS_PER_CONVERSATION = 0
"""Turns kept per conversation; 0 keeps every turn.

Override with CODETETHER_MAX_TURNS to trade corpus coverage for run time.
"""


def window(
    context: list[dict[str, object]], limit: int = CONTEXT_TURNS
) -> list[dict[str, object]]:
    """Return the trailing context plus any leading system message."""
    recent = context[-limit:]
    head = (
        context[0] if context and context[0].get('role') == 'system' else None
    )
    if head is not None and head not in recent:
        return [head, *recent]
    return recent


def stride(total: int, limit: int | None = None) -> int:
    """Return the sampling stride needed to cap turns per conversation."""
    if limit is None:
        limit = int(
            os.environ.get('CODETETHER_MAX_TURNS', MAX_TURNS_PER_CONVERSATION)
        )
    if limit <= 0 or total <= limit:
        return 1
    return (total + limit - 1) // limit
