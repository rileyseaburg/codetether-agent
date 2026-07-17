"""Conservative secret and oversized-payload screening."""

import json
import re

from .model import SourceRecord


_PATTERNS = (
    re.compile(r'AKIA[0-9A-Z]{16}'),
    re.compile(r'sk-[A-Za-z0-9_-]{20,}'),
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(
        r'(?:authorization|bearer|password|passwd|api[_-]?key|access[_-]?key|secret)'
        r'[\" :=]{1,8}[A-Za-z0-9+/=_-]{16,}',
        re.IGNORECASE,
    ),
)


def rejection_reason(
    record: SourceRecord, max_content_chars: int
) -> str | None:
    """Return a safety rejection reason, if one applies."""
    serialized = json.dumps(record.value, sort_keys=True)
    content = record.value.get('content')
    if isinstance(content, str) and len(content) > max_content_chars:
        return 'oversized_content'
    if 'data:image/' in serialized and ';base64,' in serialized:
        return 'embedded_image'
    if any(pattern.search(serialized) for pattern in _PATTERNS):
        return 'possible_secret'
    return None
