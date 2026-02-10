# Release Notes

## v1.1.0 — Security-First Release

**Date:** 2026-02-10

This release implements four major security features that make CodeTether's marketing claims real: mandatory auth, system-wide audit logging, plugin sandboxing with code signing, and Kubernetes self-deployment.

### New Features

#### Mandatory Authentication Middleware (`src/server/auth.rs`)
- Bearer token authentication layer that **cannot be disabled**
- Auto-generates HMAC-SHA256 token from hostname + timestamp if `CODETETHER_AUTH_TOKEN` is not set
- Exempts only `/health` endpoint
- Returns structured JSON 401 errors

#### System-Wide Audit Trail (`src/audit/mod.rs`)
- Every API call, tool execution, and session event is logged
- Append-only JSON Lines file backend
- Queryable with filters (actor, action, resource, time range)
- Global singleton via `AUDIT_LOG` — initialized once at server startup
- New endpoints: `GET /v1/audit/events`, `POST /v1/audit/query`

#### Plugin Sandboxing & Code Signing (`src/tool/sandbox.rs`)
- Tool manifest system with SHA-256 content hashing
- Ed25519 signature verification for all plugin manifests
- Sandbox policies: Default, Restricted, Custom
- Resource limits: max memory, max CPU seconds, network allow/deny
- `ManifestStore` registry for signed tool manifests

#### Kubernetes Self-Deployment (`src/k8s/mod.rs`)
- Auto-detects cluster via `KUBERNETES_SERVICE_HOST`
- Self-info from Kubernetes Downward API
- `ensure_deployment()` creates or updates Deployments
- `scale()` adjusts replica count
- `health_check()` with rolling restart of unhealthy pods
- `reconcile_loop()` background task running every 30 seconds
- New endpoints: `GET /v1/k8s/status`, `POST /v1/k8s/scale`, `POST /v1/k8s/health`, `POST /v1/k8s/reconcile`

### Dependencies Added
- `kube` 0.98 (runtime, client, derive)
- `k8s-openapi` 0.22 (v1_31)
- `sha2` 0.10
- `ed25519-dalek` 2
- `base64` 0.22

### Bug Fixes
- Fixed pre-existing lifetime bug in `a2a/worker.rs` (`catalog_alias` closure)

---

## v1.0.5

- Release with cognition runtime and core agent functionality

## v1.0.4

- Cognition runtime fixes: output normalization, deterministic uncertainties, batch locks, governance quorum/timeout, backend-aware timeout, retry classification
- 34/34 tests passing
