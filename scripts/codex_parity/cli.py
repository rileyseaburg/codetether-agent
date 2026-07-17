"""Command-line interface for the differential benchmark."""

from argparse import ArgumentParser, Namespace
from datetime import datetime, timezone
from pathlib import Path

from .manifest import load
from .runner import dry_run, run
from .spec import AgentSpec


def main() -> None:
    """Parse CLI options and run or preview the benchmark."""
    args = _parser().parse_args()
    cases = load(Path(args.manifest))
    specs = [
        AgentSpec("codex", args.codex_bin, args.codex_model, args.max_steps),
        AgentSpec("codetether", args.codetether_bin, args.codetether_model, args.max_steps),
    ]
    if args.dry_run:
        print(dry_run(cases, specs))
        return
    root = Path(args.output) if args.output else _default_output()
    print(run(cases, specs, root))


def _parser() -> ArgumentParser:
    parser = ArgumentParser(description="Compare Codex and CodeTether on identical tasks")
    parser.add_argument("--manifest", default="benchmarks/codex-parity/cases.toml")
    parser.add_argument("--codex-bin", default="codex")
    parser.add_argument("--codetether-bin", default="codetether")
    parser.add_argument("--codex-model", required=True)
    parser.add_argument("--codetether-model", required=True)
    parser.add_argument("--max-steps", type=int, default=50)
    parser.add_argument("--output")
    parser.add_argument("--dry-run", action="store_true")
    return parser


def _default_output() -> Path:
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    return Path("artifacts") / "codex-parity" / stamp
