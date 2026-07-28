"""Build the worker pool specification for a Vertex custom training job."""

from dataclasses import dataclass

from .vertex_command import container_command
from .vertex_scheduling import spot_scheduling


BOOT_DISK_GB = 500
"""The 30B base model is 61 GB, plus checkpoints and a merged export."""


@dataclass(frozen=True)
class JobRequest:
    """Inputs that determine a Vertex worker pool."""

    image: str
    machine: str
    accelerator: str
    environment: dict[str, str]
    count: int = 1
    preemptible: bool = False


def build(request: JobRequest) -> dict[str, object]:
    """Return a single worker pool running the training entrypoint."""
    spec: dict[str, object] = {'workerPoolSpecs': [_pool(request)]}
    if request.preemptible:
        spec['scheduling'] = spot_scheduling()
    return spec


def _pool(request: JobRequest) -> dict[str, object]:
    """Return the single worker pool definition."""
    return {
        'machineSpec': {
            'machineType': request.machine,
            'acceleratorType': request.accelerator,
            'acceleratorCount': request.count,
        },
        'replicaCount': 1,
        'diskSpec': {
            'bootDiskType': 'pd-ssd',
            'bootDiskSizeGb': BOOT_DISK_GB,
        },
        'containerSpec': {
            'imageUri': request.image,
            'command': container_command(),
            'env': [
                {'name': key, 'value': value}
                for key, value in sorted(request.environment.items())
            ],
        },
    }
