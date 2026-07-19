"""Make Forgejo task identity derivation independent of DB configuration."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_task.py")
IMPORT = "from . import database as db\n"
ANCHOR = """async def prepare(metadata: Dict[str, Any]) -> Tuple[str, Optional[Dict[str, Any]]]:
    \"\"\"Require durable storage, bind the worker, and find any prior task.\"\"\"
"""
REPLACEMENT = """async def prepare(metadata: Dict[str, Any]) -> Tuple[str, Optional[Dict[str, Any]]]:
    \"\"\"Require durable storage, bind the worker, and find any prior task.\"\"\"
    from . import database as db
"""


def main() -> None:
    """Move the database import into the persistence operation."""
    text = PATH.read_text()
    if REPLACEMENT in text and IMPORT not in text:
        return
    if text.count(IMPORT) != 1 or text.count(ANCHOR) != 1:
        raise RuntimeError("lazy database import anchors are missing or ambiguous")
    text = text.replace(IMPORT, "", 1)
    PATH.write_text(text.replace(ANCHOR, REPLACEMENT, 1))


if __name__ == "__main__":
    main()