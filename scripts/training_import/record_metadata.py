"""Stable provenance metadata for imported session messages."""

from .time_order import timestamp


def build(session_id: str, created: object, index: int) -> dict[str, object]:
    """Return metadata compatible with training-v2 correlation."""
    return {
        'bus_kind': 'agent_message',
        'envelope_id': f'vm-{session_id}-{index}',
        'timestamp': timestamp(created, index),
        'topic': 'vm/session-import',
        'sender_id': 'vm-session-import',
        'correlation_id': session_id,
        'step': index,
    }
