"""Bound supervision context so rendering stays linear in corpus size.

Some conversations exceed 1,600 messages. Rendering every prefix is
quadratic, so each assistant turn keeps only a recent context window and
very long conversations contribute a bounded number of turns.
"""

CONTEXT_TURNS = 12
MAX_TURNS_PER_CONVERSATION = 24


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


def stride(total: int, limit: int = MAX_TURNS_PER_CONVERSATION) -> int:
    """Return the sampling stride needed to cap turns per conversation."""
    if total <= limit:
        return 1
    return (total + limit - 1) // limit
