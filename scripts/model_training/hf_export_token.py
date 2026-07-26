"""Print the HuggingFace token for shell export.

Base-model downloads are large enough that anonymous requests get
rate-limited, so the training shell needs `HF_TOKEN` populated from Vault.
The value is written only to stdout for immediate capture, never to disk.
"""

from .hf_token import resolve_token


def main() -> None:
    """Write the resolved token to stdout with no trailing newline."""
    print(resolve_token(), end='')


if __name__ == '__main__':
    main()
