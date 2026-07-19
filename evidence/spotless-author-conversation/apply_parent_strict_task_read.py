"""Make deterministic task lookup distinguish storage errors from absence."""

from pathlib import Path

DATABASE = Path('/home/riley/A2A-Server-MCP/a2a_server/database.py')
TEST = Path('/home/riley/A2A-Server-MCP/tests/test_forgejo_author_prepare.py')


def main() -> None:
    """Propagate task-read errors and prove author creation aborts."""
    text = DATABASE.read_text()
    old = '''    except Exception as e:
        logger.error(f'Failed to get task: {e}')
        return None


async def db_list_tasks(
'''
    new = '''    except Exception as e:
        logger.error(f'Failed to get task: {e}')
        raise


async def db_list_tasks(
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('strict task read anchor is missing')
        DATABASE.write_text(text.replace(old, new, 1))
    test = TEST.read_text()
    addition = '''

@pytest.mark.asyncio
async def test_prepare_propagates_durable_read_failures(monkeypatch):
    install_database(monkeypatch)

    async def failed_read(_task_id):
        raise OSError('database read failed')

    monkeypatch.setattr(database, 'db_get_task', failed_read)
    with pytest.raises(OSError, match='database read failed'):
        await prepare(metadata())
'''
    if 'test_prepare_propagates_durable_read_failures' not in test:
        TEST.write_text(test.rstrip() + addition)


if __name__ == '__main__':
    main()