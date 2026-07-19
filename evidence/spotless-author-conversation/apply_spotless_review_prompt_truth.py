"""Keep the author prompt truthful about untrusted reviewer output."""

from pathlib import Path

PAYLOAD = Path(
    '/home/riley/spotlessbinco/scripts/codetether-review/author_payload.sh'
)


def main() -> None:
    """Remove an unsupported reviewer-attestation claim from the prompt."""
    text = PAYLOAD.read_text()
    text = text.replace(
        'A cryptographically verified reviewer returned the diagnostic below. ',
        'A configured CodeTether reviewer returned the diagnostic below. ',
        1,
    )
    PAYLOAD.write_text(text)


if __name__ == '__main__':
    main()