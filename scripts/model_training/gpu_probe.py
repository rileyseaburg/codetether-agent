"""Report GPU capacity and the model size it can train.

Run before training so an undersized device fails fast with a clear
recommendation instead of an opaque out-of-memory traceback mid-run.
"""

import json

import torch

from .vram_plan import plan


def main() -> None:
    """Print device capacity and the recommended base model."""
    if not torch.cuda.is_available():
        message = 'CUDA unavailable: training requires an NVIDIA GPU'
        raise SystemExit(message)
    properties = torch.cuda.get_device_properties(0)
    gigabytes = properties.total_memory / 1e9
    recommendation = plan(gigabytes)
    print(
        json.dumps(
            {
                'gpu': properties.name,
                'memory_bytes': properties.total_memory,
                'gigabytes': round(gigabytes, 1),
                'cuda': torch.version.cuda,
                **recommendation,
            },
            sort_keys=True,
        )
    )


if __name__ == '__main__':
    main()
