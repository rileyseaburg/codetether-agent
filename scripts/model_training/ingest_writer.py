"""Write a deduplicated multi-source conversation corpus."""

from pathlib import Path

from .codex_rollout import messages as codex_messages
from .ingest_sink import Sink
from .ingest_stats import Stats
from .session_file import messages as session_messages


def ingest(
    stores: list[Path], codex: Path, output: Path, min_assistant: int
) -> dict[str, object]:
    """Ingest every store plus Codex rollouts into one JSONL file."""
    stats = Stats()
    with output.open('wb') as handle:
        sink = Sink(handle, stats, min_assistant)
        for store in stores:
            _ingest_sessions(sink, store)
        for path in sorted(codex.rglob('*.jsonl')):
            sink.emit(codex_messages(str(path)), path, 'codex')
    return stats.report(str(output))


def _ingest_sessions(sink: Sink, store: Path) -> None:
    """Ingest every CodeTether session file in one store."""
    try:
        paths = sorted(store.glob('*.json'))
    except OSError:
        return
    for path in paths:
        sink.emit(session_messages(path), path, 'codetether')
