# v2.0.1

## What's New

- **MiniMax Anthropic Provider Harness** — New provider implementation supporting MiniMax models through the Anthropic-compatible API interface
- **Prompt Caching** — Anthropic provider now supports prompt caching to reduce latency and token costs on repeated context
- **Protocol-First Autochat Relay CLI** — New `run` subcommand with autochat relay capabilities for protocol-driven conversation flows

## Changes

- Expanded `run` CLI with 500+ lines of new autochat relay functionality
- Enhanced Anthropic provider with caching support and improved harness architecture
- Updated provider registry to accommodate new MiniMax integration
- Minor adjustments to A2A worker, session handling, and TUI modules

**Stats:** 7 files changed, 1,122 insertions(+), 76 deletions(-)
