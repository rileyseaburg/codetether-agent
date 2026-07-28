"""Verify supervision covers completions only, before spending GPU hours.

The v2 model trained loss on every token, including user and system text,
and regressed to a 0.00 code-fix pass rate. This audit inspects the label
tensors TRL will build, so a masking regression is caught in seconds rather
than after a full epoch.

A small fraction of fully masked examples is expected: prompts near the
sequence limit lose their completion to truncation, and TRL drops those rows
itself. Only a systemic failure should stop a run, so the gate trips on a
rate rather than on any single occurrence. Blocking at the first occurrence
previously failed a healthy run where 5 of 200 sampled pairs, 2.5 percent,
exceeded the window.
"""

import argparse
import json

from pathlib import Path

from transformers import AutoTokenizer

from .constants import BASE_MODEL, BASE_REVISION
from .mask_gate import gate
from .mask_report import audit


def main() -> None:
    """Print masking statistics for a sample of rendered pairs."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--pairs', type=Path, required=True)
    parser.add_argument('--sample', type=int, default=200)
    parser.add_argument('--max-length', type=int, default=8192)
    parser.add_argument('--output', type=Path)
    values = parser.parse_args()
    tokenizer = AutoTokenizer.from_pretrained(
        BASE_MODEL, revision=BASE_REVISION, use_fast=True
    )
    report = audit(values.pairs, tokenizer, values.sample, values.max_length)
    text = json.dumps(report, indent=2, sort_keys=True) + '\n'
    if values.output:
        values.output.write_text(text)
    print(text)
    gate(report)


if __name__ == '__main__':
    main()
