"""Benchmark local tool-calling and code-correction behavior."""

import argparse
import json

from pathlib import Path

from .bench_probes import code_probes, tool_probes


def main() -> None:
    """Probe the local server and persist benchmark evidence."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--base-url', default='http://127.0.0.1:8099')
    parser.add_argument('--model', default='codetether-local')
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    tools = tool_probes(values.base_url, values.model)
    code = code_probes(values.base_url, values.model)
    summary = {
        'code_cases': code,
        'code_pass_rate': sum(1 for c in code if c['passed']) / len(code),
        'tool_call_rate': sum(1 for t in tools if t['emitted_tool_calls'])
        / len(tools),
        'tool_cases': tools,
    }
    values.output.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + '\n'
    )
    print(
        json.dumps(
            {k: summary[k] for k in ('tool_call_rate', 'code_pass_rate')}
        )
    )


if __name__ == '__main__':
    main()
