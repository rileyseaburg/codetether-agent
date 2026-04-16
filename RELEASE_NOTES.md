]633;E;echo "# v4.5.0";d75f3921-77b4-4ba3-8bce-bfebee5f1e5a]633;C# v4.5.0

## What's New

### macOS CI Builds on Local Mac Mini
- **Jenkins now builds native macOS binaries** (arm64 + x86_64) on the local Mac Mini via SSH, replacing GitHub Actions macOS runners
- Uses `sshpass` with password-based auth and `usernamePassword` Jenkins credential (`mac-mini-ssh`)
- macOS build stage is gated by `buildingTag()` — only runs on release tags to avoid blocking branch builds
- Removed `.github/workflows/macos-release.yml` since macOS builds moved to Jenkins

### Browser Control Module Refactored
- Split the 542-line `src/tool/browserctl/mod.rs` into 9 SRP-focused modules: `input`, `helpers`, `http`, `response`, `schema`, `dispatch`, and `actions/{nav,dom,eval,device,tabs,lifecycle}`

### Release Pipeline Improvements
- Added `cargo publish` step to `release.sh` with dry-run validation before publishing
- Proxmox API token (`codetether-ci`) created and stored in Vault for infrastructure automation

## Bug Fixes
- Fixed transient LLM network errors with retry logic (#33)
- Fixed panic on non-char-boundary byte slice in provider error messages
- Fixed worktree branch leak — branches now deleted after cleanup
- Fixed event loop tick starvation by replacing `tokio::time::sleep` with `Interval`
- Fixed watchdog losing steering after agent swap — stores `effective_prompt` instead of raw input
- Removed unused imports across TUI, session, and server modules
- Removed dead `branch_cleanup.rs` and unlinked `runtime_env.rs`

## Changes
- 41 files changed, 1,116 insertions(+), 765 deletions(-)
- Merged PR #53 (browserctl SRP split + review feedback)
- Closed PR #51 (superseded), PR #52 (too large, feedback provided)
