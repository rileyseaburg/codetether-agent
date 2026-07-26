"""Stub device probe used to verify bootstrap path wiring off-GPU."""

import json


def main() -> None:
    """Emit a small-model plan so no large download is attempted."""
    print(
        json.dumps(
            {
                'gpu': 'stub',
                'recommended_model': 'Qwen/Qwen3-4B-Instruct-2507',
                'max_length': 2048,
            }
        )
    )


if __name__ == '__main__':
    main()
