"""Apply absolute imports and stable database monkeypatching in protocol code."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
MODULES = [
    ROOT / 'a2a_server/agent_task_record.py',
    ROOT / 'a2a_server/agent_task_persistence.py',
    *ROOT.glob('a2a_server/forgejo_*.py'),
    ROOT / 'a2a_server/worker_target_validation.py',
]


def absolute_imports() -> None:
    """Replace package-relative imports in focused production modules."""
    for path in MODULES:
        text = path.read_text()
        text = text.replace('from . import database as db',
            'from a2a_server import database as db')
        text = text.replace('from .', 'from a2a_server.')
        local = '    from a2a_server import database as db\n'
        if local in text:
            text = text.replace(local, '')
            if 'from a2a_server import database as db\n' not in text:
                marker = '\nfrom a2a_server.'
                index = text.find(marker)
                if index < 0:
                    raise RuntimeError(f'database import anchor missing: {path}')
                text = text[:index] + '\nfrom a2a_server import database as db' + text[index:]
        path.write_text(text)


def patch_prepare_test() -> None:
    """Patch database functions rather than replacing the imported module."""
    path = ROOT / 'tests/test_forgejo_author_prepare.py'
    text = path.read_text().replace('import a2a_server\n',
        'from a2a_server import database\n')
    old = '''    database = SimpleNamespace(
        get_pool=get_pool,
        db_get_task=get_task,
        db_get_active_worker_by_name=get_worker,
    )
    monkeypatch.setitem(a2a_server.__dict__, 'database', database)
'''
    new = '''    monkeypatch.setattr(database, 'get_pool', get_pool)
    monkeypatch.setattr(database, 'db_get_task', get_task)
    monkeypatch.setattr(database, 'db_get_active_worker_by_name', get_worker)
'''
    path.write_text(text.replace(old, new))


def patch_lock_test() -> None:
    """Keep lock tests attached to the production database module object."""
    path = ROOT / 'tests/test_forgejo_author_lock.py'
    text = path.read_text().replace('from types import SimpleNamespace\n\n', '')
    text = text.replace('import a2a_server\n', 'from a2a_server import database\n')
    text = text.replace(
        '''    monkeypatch.setitem(
        a2a_server.__dict__, 'database', SimpleNamespace(get_pool=no_pool)
    )
''',
        "    monkeypatch.setattr(database, 'get_pool', no_pool)\n",
    ).replace(
        '''    monkeypatch.setitem(
        a2a_server.__dict__, 'database', SimpleNamespace(get_pool=get_pool)
    )
''',
        "    monkeypatch.setattr(database, 'get_pool', get_pool)\n",
    )
    path.write_text(text)


def patch_tenant_test() -> None:
    """Patch tenant lookup functions on the imported database module."""
    path = ROOT / 'tests/test_forgejo_tenant_binding.py'
    text = path.read_text().replace('import a2a_server\n',
        'from a2a_server import database\n')
    old = '''    database = SimpleNamespace(
        get_pool=get_pool,
        db_get_task=get_task,
        db_get_active_worker_by_name=get_worker,
    )
    monkeypatch.setitem(a2a_server.__dict__, 'database', database)
'''
    new = '''    monkeypatch.setattr(database, 'get_pool', get_pool)
    monkeypatch.setattr(database, 'db_get_task', get_task)
    monkeypatch.setattr(database, 'db_get_active_worker_by_name', get_worker)
'''
    path.write_text(text.replace(old, new))


def main() -> None:
    """Apply import and test-isolation changes."""
    absolute_imports()
    patch_prepare_test()
    patch_lock_test()
    patch_tenant_test()


if __name__ == '__main__':
    main()