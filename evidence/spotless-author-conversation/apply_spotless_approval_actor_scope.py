"""Declare the configured approval actor before assigning it."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/scripts/codetether-review/approval.sh")
OLD = '''  local token="$6" temp_dir="$7" enabled pr_file reviews_file payload response
'''
NEW = '''  local token="$6" temp_dir="$7" enabled pr_file reviews_file payload response actor
'''
LATE = '''  local current_head head_repo base_repo marker actor active_id dismissed_id
'''
FIXED = '''  local current_head head_repo base_repo marker active_id dismissed_id
'''


def main() -> None:
    """Apply both sides of the actor-scope correction."""
    text = PATH.read_text()
    if OLD in text:
        text = text.replace(OLD, NEW, 1)
    if LATE in text:
        text = text.replace(LATE, FIXED, 1)
    if NEW not in text or FIXED not in text:
        raise RuntimeError("approval actor scope anchors are missing")
    PATH.write_text(text)


if __name__ == "__main__":
    main()