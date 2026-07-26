"""Identify assistant turns across heterogeneous transcript formats."""

ASSISTANT = {'assistant', 'agent', 'model'}


def is_assistant(record: dict[str, object]) -> bool:
    """Return whether a transcript record carries assistant output."""
    if str(record.get('role', '')).lower() in ASSISTANT:
        return True
    payload = record.get('payload')
    if isinstance(payload, dict):
        if str(payload.get('role', '')).lower() in ASSISTANT:
            return True
        if payload.get('type') == 'agent_message':
            return True
    message = record.get('message')
    if isinstance(message, dict):
        return str(message.get('role', '')).lower() in ASSISTANT
    return False
