# v1.1.6-alpha-8.5

## What's New

- **Protocol-first autochat relay** — New relay infrastructure (`src/bus/relay.rs`) enables protocol-first communication patterns for improved agent orchestration
- **Installer version checking** — Install scripts now validate and report version information during setup
- **Model discovery in installer** — Installers can discover and configure available models during installation
- **Protocol registry UX** — Enhanced user experience for protocol registration and management in the TUI

## Bug Fixes

- Removed accidentally committed secrets from `jenkinsfile.sh`
- Fixed case sensitivity issue in `.gitignore`

## Changes

- **TUI improvements** — Major TUI expansion with 1000+ new lines for enhanced terminal interface capabilities
- **Installer script updates** — Significant improvements to both `install.sh` and `install.ps1` with better error handling and model configuration
- **Bus infrastructure** — New `registry.rs` module for managing protocol endpoints and message routing
- **Cognition system refinements** — Updated persistence and thinker modules for improved agent reasoning
- **Server and provider updates** — Minor adjustments to server routing and Bedrock provider integration

**Stats:** 14 files changed, 1,965 insertions(+), 221 deletions(-)
