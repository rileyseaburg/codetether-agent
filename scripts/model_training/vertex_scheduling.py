"""Scheduling policy for preemptible Vertex training jobs.

This project has zero non-preemptible A100 40 GB training quota; submitting
on demand returned HTTP 429 RESOURCE_EXHAUSTED for
`custom_model_training_nvidia_a100_gpus`. The available quotas are
preemptible A100 (limit 8) and A100 80 GB (limit 8).
"""

MAX_WAIT_SECONDS = 86400
RESTART_ON_WORKER_RESTART = True
"""Spot capacity is reclaimed without warning.

Checkpoints stream to Cloud Storage every five minutes, so a restart resumes
from the last checkpoint rather than starting over.
"""


def spot_scheduling() -> dict[str, object]:
    """Return scheduling that tolerates preemption."""
    return {
        'strategy': 'SPOT',
        'restartJobOnWorkerRestart': RESTART_ON_WORKER_RESTART,
        'timeout': f'{MAX_WAIT_SECONDS}s',
    }
