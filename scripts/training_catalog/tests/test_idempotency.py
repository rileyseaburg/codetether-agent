"""Idempotency tests for Polaris grants and assignments."""

import unittest

from training_catalog.assignments import ensure_named
from training_catalog.grants import ensure_catalog_grant
from training_catalog.resources import Resources


class FakeResources(Resources):
    """Record writes while returning a supplied list response."""

    def __init__(self, body: object) -> None:
        self.body = body
        self.writes: list[tuple[str, object]] = []

    def get(self, path: str) -> object:
        return self.body

    def put(self, path: str, payload: object) -> None:
        self.writes.append((path, payload))


class IdempotencyTests(unittest.TestCase):
    """Ensure an existing Polaris relationship is never rewritten."""

    def test_existing_grant_is_not_rewritten(self) -> None:
        resources = FakeResources(
            {
                'grants': [
                    {
                        'type': 'catalog',
                        'privilege': 'CATALOG_MANAGE_CONTENT',
                    }
                ]
            }
        )
        ensure_catalog_grant(resources, '/grants')
        self.assertEqual(resources.writes, [])

    def test_missing_assignment_is_written(self) -> None:
        resources = FakeResources({'roles': []})
        payload = {'principalRole': {'name': 'writer'}}
        ensure_named(resources, '/roles', 'writer', payload)
        self.assertEqual(resources.writes, [('/roles', payload)])


if __name__ == '__main__':
    unittest.main()
