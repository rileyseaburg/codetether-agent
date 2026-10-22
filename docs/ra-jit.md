# First-party Rust development tools

CodeTether's `rust` command is the shared entry point for the VS Code extension
and native agent workflow. It uses ra-jit for explicit build/run/test actions,
and normal Cargo for `check`. It does **not** replace rust-analyzer, share its
analysis database, or promise faster completions, navigation, or diagnostics.

## Prerequisites

Use a CodeTether binary built with this integration, Cargo/rustup, Rust 1.99.0
with rust-src, and ra-jit installed on the executing machine. For SSH/remote
VS Code, this is the remote extension host, not the local desktop.

```sh
# Explicit user setup, never performed automatically by the integration:
rustup toolchain install 1.99.0 --component rust-src
cargo +1.99.0 install --locked ra-jit
codetether rust --cwd /path/to/workspace --toolchain 1.99.0 doctor
```

The default toolchain is `stable`; ra-jit actions reject it if rustc is no longer
1.99.0. Pin `--toolchain 1.99.0` for repeatable behavior. Missing prerequisites
and nonzero subprocess exits are errors, not silent success or automatic setup.

## Terminal and editor

```sh
codetether rust --cwd /path/to/workspace --package app --bin app build
codetether rust --cwd /path/to/workspace status
codetether rust --cwd /path/to/workspace --package app --bin app run --arg=--help
codetether rust --cwd /path/to/workspace --package app check
```

`extensions/codetether-rust` contains the first-party VS Code extension. Run it
in an Extension Development Host with `code --extensionDevelopmentPath=<absolute
path to extensions/codetether-rust>`, or package with `npx @vscode/vsce package`
from that directory and install the VSIX. It requires a trusted workspace and
uses VS Code process tasks (no shell), task output, cancellation, and its own
`$codetether-rustc` matcher. Diagnostics appear under source **CodeTether Rust**
in Problems, including for open files. A failed task opens Problems and reports
its exit status. Select **CodeTether Rust: Build (ra-jit)** from the palette.
Multi-root windows prompt for the target folder; that folder must contain the
Cargo.toml to use. It does not install tools or enable checks on save automatically.

Configure `codetether.rust.package`, `.bin`, `.toolchain`, and literal `.runArgs`
as needed. The machine-scoped `.executable` setting selects the CodeTether binary.
The separate **Check (rustc)** command preserves borrow checking. Stop prompts
because the ra-jit daemon is shared by other workspaces for the same user.

## Native agent

Call the registered `rust_build_plan` tool with:

```json
{"action":"build","cwd":"/absolute/workspace","package":"app","bin":"app"}
```

The tool returns `executed: false`, literal argv and suggested `exec_command`
arguments. Submit those arguments through the ordinary native `exec_command`
tool with an appropriate justification; planning is **not approval**. Poll a
returned session ID using `write_stdin`. No separate process/session registry
or hidden execution bypasses the existing sandbox, access mode or command guards.

Builds, checks, tests and programs may execute arbitrary workspace code. Never
infer read-only execution from the planner's read-only classification. No grant
or approval identifier is accepted by the planner. Normal Cargo/CI remains the
correctness authority: ra-jit has no borrow checker. First builds and unsupported
edits may fall back to Cargo; inspect actual task output and `status`. A fast
no-change build is not evidence of faster function-edit builds or editor latency.