"""Survey the durable artifacts a training run should produce."""

from pathlib import Path


def survey(state: Path) -> dict[str, object]:
    """Return log, checkpoint, and adapter presence under a state root."""
    output = state / 'output'
    log = state / 'logs' / 'train.log'
    checkpoints = sorted(
        (p.name for p in output.glob('checkpoint-*') if p.is_dir()),
        key=_step,
    )
    return {
        'state': str(state),
        'state_exists': state.exists(),
        'log_exists': log.exists(),
        'log_bytes': log.stat().st_size if log.exists() else 0,
        'checkpoints': checkpoints,
        'latest_checkpoint': checkpoints[-1] if checkpoints else None,
        'final_adapter': (output / 'final-adapter').is_dir(),
    }


def _step(name: str) -> int:
    """Return the numeric step from a checkpoint directory name."""
    tail = name.rsplit('-', 1)[-1]
    return int(tail) if tail.isdigit() else 0
