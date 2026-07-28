# Contributing

Thanks for working on tetherscript. Keep changes focused, reviewable, and
aligned with the runtime semantics already in the repository.

## Development Rules

- Use lowercase `tetherscript` in user-facing language copy.
- Keep the language dynamically typed and runtime ownership checked.
- Do not add dependencies without a concrete design reason.
- Do not replace the native browser agent path with Chromium, Chrome DevTools,
  or Playwright.
- Add tests for every behavior change.
- Preserve the examples unless the change intentionally updates their behavior.

## Required Checks

Run these before sending a release or pull request:

```bash
bash ./check_file_limits.sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo package
```

On Windows without Bash, run the equivalent PowerShell checker:

```powershell
.\check_file_limits.ps1
```

For local feature work, also run the relevant `.tether` examples through the
`tetherscript` binary.

## Release Checklist

- Bump the crate version in `Cargo.toml` and `Cargo.lock`.
- Confirm `cargo publish --dry-run` passes.
- Confirm the package contents are intentional with `cargo package --list`.
- Publish only from a clean, committed worktree.
