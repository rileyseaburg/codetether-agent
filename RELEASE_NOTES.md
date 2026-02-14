# v2.1.0

## What's New

**RLM Integration for Session Context Management** — Sessions now automatically use Recursive Language Model (RLM) processing to compress and manage context, enabling longer conversations without hitting token limits.

**Auth Registration & Credential Auto-Load** — New `auth register` command streamlines setup by automatically detecting and loading credentials from environment variables and configuration files.

**OKR Correctness Hardening** — Enhanced Objectives and Key Results (OKR) tracking with comprehensive validation and persistence layer, improving reliability of goal-tracking workflows.

**Worker HTTP Server for Kubernetes** — Workers now expose an HTTP server with `/health` and `/ready` endpoints, enabling native Kubernetes liveness/readiness probes and Knative integration.

**Container Support** — Added Dockerfile and GitHub Actions workflow for building and publishing container images, simplifying cloud deployments.

## Bug Fixes

- Improved MCP transport reliability with enhanced error handling

## Changes

- **TUI**: Significant expansion with 700+ new lines of functionality
- **Session**: Enhanced session management with RLM integration
- **Server**: Extended server capabilities with additional endpoints
- **Tests**: Added comprehensive test suites for OKR correctness and model resolution
- **Documentation**: Updated README with expanded deployment and usage examples

---

**Stats**: 24 files changed, 4,932 insertions(+), 263 deletions(-)
