"""Strict JSONL parsing with lossless rejection capture."""

import json

from typing import cast

from .model import JsonObject, RejectedRecord, SourceRecord


def jsonl(text: str) -> tuple[list[SourceRecord], list[RejectedRecord], int]:
    """Parse non-empty JSONL lines and retain invalid source values."""
    records: list[SourceRecord] = []
    rejected: list[RejectedRecord] = []
    count = 0
    for line_number, raw in enumerate(text.splitlines(), 1):
        if not raw.strip():
            continue
        count += 1
        try:
            value = json.loads(raw)
        except json.JSONDecodeError:
            rejected.append(RejectedRecord(line_number, 'invalid_json', raw))
            continue
        if not isinstance(value, dict):
            rejected.append(
                RejectedRecord(line_number, 'non_object_json', value)
            )
            continue
        records.append(SourceRecord(line_number, cast(JsonObject, value)))
    return records, rejected, count
