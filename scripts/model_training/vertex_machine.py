"""Select a Vertex machine and accelerator for a base model.

Verified quota in us-central1 for project spotlessbinco:
`restricted_image_training_nvidia_a100_80gb_gpus` and
`custom_model_training_preemptible_nvidia_a100_gpus`, both limit 8.
"""

import os


A100_80GB = ('a2-ultragpu-1g', 'NVIDIA_A100_80GB')
A100_40GB = ('a2-highgpu-1g', 'NVIDIA_TESLA_A100')
LARGE_MARKER = '30B'

MULTI_GPU_MACHINES = {
    2: 'a2-highgpu-2g',
    4: 'a2-highgpu-4g',
    8: 'a2-highgpu-8g',
}
"""Machine types carrying multiple 40 GB A100 devices.

Quota allows eight preemptible A100 GPUs, so a full node trains the whole
corpus in roughly an eighth of the single-device time.
"""


def select(model: str) -> tuple[str, str]:
    """Return the machine type and accelerator for a model.

    The 30B mixture-of-experts model needs 45.4 GB and hit CUDA OOM on a
    40 GB device, so it requires the 80 GB tier. The dense 4B model needs
    7.4 GB and runs on either.
    """
    count = gpu_count()
    if count > 1 and LARGE_MARKER not in model:
        return MULTI_GPU_MACHINES[count], A100_40GB[1]
    return A100_80GB if LARGE_MARKER in model else A100_40GB


def gpu_count() -> int:
    """Return the requested GPU count, restricted to supported node sizes."""
    value = int(os.environ.get('CODETETHER_GPU_COUNT', '1'))
    return value if value in MULTI_GPU_MACHINES or value == 1 else 1
