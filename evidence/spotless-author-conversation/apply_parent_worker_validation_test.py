"""Write regression tests for extracted worker target validation."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_worker_target_validation.py")
CONTENT = '''import pytest
from fastapi import HTTPException

import a2a_server.worker_target_validation as validation


@pytest.mark.asyncio
async def test_strict_target_rejects_missing_worker(monkeypatch):
    async def missing(_target):
        return None

    monkeypatch.setattr(validation, '_load', missing)
    with pytest.raises(HTTPException) as error:
        await validation.validate_target_worker(
            {'target_worker_id': 'worker'}, strict=True
        )
    assert error.value.status_code == 409


@pytest.mark.asyncio
async def test_advisory_target_falls_back(monkeypatch):
    async def missing(_target):
        return None

    monkeypatch.setattr(validation, '_load', missing)
    metadata = {'target_worker_id': 'worker'}
    await validation.validate_target_worker(metadata)
    assert 'target_worker_id' not in metadata
    assert 'auto-routed' in str(metadata['_routing_warning'])


@pytest.mark.asyncio
async def test_recent_target_is_preserved(monkeypatch):
    async def active(_target):
        return {'last_seen': 'timestamp'}

    monkeypatch.setattr(validation, '_load', active)
    monkeypatch.setattr(validation, 'is_recent', lambda _value: True)
    metadata = {'target_worker_id': 'worker'}
    await validation.validate_target_worker(metadata, strict=True)
    assert metadata == {'target_worker_id': 'worker'}
'''


def main() -> None:
    """Write the deterministic extraction regression tests."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()