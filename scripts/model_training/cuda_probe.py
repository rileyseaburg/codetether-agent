"""Report CUDA device properties that constrain kernel design.

Occupancy, tile sizes, and shared-memory budgets all follow from these
numbers, so they are read from the device rather than assumed.
"""

import json

import torch


def main() -> None:
    """Print device limits relevant to writing CUDA kernels."""
    if not torch.cuda.is_available():
        raise SystemExit('no CUDA device visible')
    p = torch.cuda.get_device_properties(0)
    report = {
        'name': p.name,
        'capability': f'{p.major}.{p.minor}',
        'sm_count': p.multi_processor_count,
        'shared_per_block_kb': p.shared_memory_per_block // 1024,
        'shared_per_sm_kb': p.shared_memory_per_multiprocessor // 1024,
        'regs_per_sm': p.regs_per_multiprocessor,
        'warp_size': p.warp_size,
        'total_memory_gb': round(p.total_memory / 1e9, 2),
        'l2_cache_mb': round(getattr(p, 'l2_cache_size', 0) / 1e6, 2),
        'max_threads_per_sm': p.max_threads_per_multi_processor,
        'torch': torch.__version__,
        'cuda': torch.version.cuda,
        'bf16': torch.cuda.is_bf16_supported(),
    }
    print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == '__main__':
    main()
