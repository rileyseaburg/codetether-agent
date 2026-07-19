# Forgejo Reviewer-to-Author Protocol

`codetether.forgejo-author.v1` routes a review back to the agent that authored a
pull request without trusting an arbitrary agent name from workflow input.

## Identity contract

The author agent and review workflow independently derive the route:

```text
ctforgejo_ + first_40_hex(sha256(
  lowercase(forgejo_host) + "\n" +
  lowercase(forgejo_login) + "\n" +
  lowercase(agent_slot)
))
```

Configure every author worker with:

```bash
export CODETETHER_FORGEJO_HOST=forge.example
export CODETETHER_FORGEJO_LOGIN=alice
export CODETETHER_AGENT_SLOT=default
export CODETETHER_TENANT_ID=tenant-a
export CODETETHER_KEY_ID=author-key-a
export CODETETHER_SIGNING_SECRET='loaded-from-a-secret-store'
```

The worker advertises the derived identity and commit provenance emits these
signed trailers:

```text
CodeTether-Agent-Identity: ctforgejo_...
CodeTether-Forgejo-Host: forge.example
CodeTether-Forgejo-Login: alice
CodeTether-Agent-Slot: default
CodeTether-Session-ID: reusable_session_id
CodeTether-Provenance-ID: ctprov_...
CodeTether-Tenant-ID: tenant-a
CodeTether-Key-ID: author-key-a
CodeTether-Signature: hmac_sha256_hex
```

The commit must be signed by a key registered to the Forgejo login. The trusted
base workflow asks Forgejo to verify the exact head SHA and requires the
reported signer to equal both the PR author and the signed login trailer. It
does not use a repository-pinned author key or local `git verify-commit`
fallback. The server independently fetches that exact commit from Forgejo and
requires every identity, session, and provenance field in the request to equal
one unambiguous trailer in the verified commit message.

The CodeTether signature is a second, independent proof. It is an HMAC over the
complete provenance envelope, including the session, tenant, and canonical
agent identity. Thus, a Forgejo user cannot authorize an arbitrary CodeTether
session merely by placing its identifier in a signed commit.

## Server enforcement

The server rejects the request unless all of the following hold:

1. The protocol, source, and workflow-stage labels are exact.
2. The head SHA, provenance ID, and reusable session ID are well formed.
3. The requested target equals the independently derived canonical identity.
4. Workspace preservation and provenance verification are explicitly enabled.
5. The provenance HMAC resolves to a configured key bound to the canonical
   identity and tenant.
6. An active worker proves possession of that same key when registering,
   unregistering, and on every liveness heartbeat. Registration and lifecycle
   changes bind their exact JSON bytes; task-specific proofs bind claims and
   every status, output, release, resume, and progress mutation to its action,
   task ID, durable claim owner, active state, and exact JSON-body SHA-256.
   The first canonical claim performs a conditional PostgreSQL reservation, so
   two API replicas cannot acknowledge different workers for the same task.
7. Forgejo independently confirms the open PR head, author, and commit signer.
8. Durable PostgreSQL task storage is available.
9. The CodeTether bearer token resolves to a server-controlled replay scope.
   The same tenant or labeled-token principal is required for task reads,
   output streams, listings, and cancellation.

The workflow sends its event-scoped Forgejo credential in `X-Forgejo-Token`.
The server uses it only for verification; it is never copied into task metadata.
Allowlist Forgejo endpoints with a JSON host map:

```bash
export CODETETHER_FORGEJO_API_BASE_URLS='{ "forge.example": "https://forge.example/api/v1" }'
```

Configure the server-authoritative HMAC registry as JSON. Load it from a secret
manager, never from repository source. Give each canonical worker only its own
`CODETETHER_KEY_ID`, `CODETETHER_SIGNING_SECRET`, and
`CODETETHER_TENANT_ID`; do not distribute the full registry to workers:

```bash
export CODETETHER_PROVENANCE_SIGNING_KEYS='{
  "author-key-a": {
    "secret": "shared-with-that-worker-only",
    "agent_identity": "ctforgejo_...",
    "tenant_id": "tenant-a",
    "task_auth_label": "forgejo-reviewer"
  }
}'
```

Configure the worker principal with `CODETETHER_FORGEJO_HOST`,
`CODETETHER_FORGEJO_LOGIN`, and `CODETETHER_AGENT_SLOT`. The Rust worker derives
the canonical route from those values; a repository never supplies the route.

The logical task ID is derived from the trusted key's tenant, Forgejo host,
repository, PR number, stage, immutable head SHA, and canonical author route.
Client metadata cannot override this scope. Worker registration, heartbeat,
lookup, task storage, task claiming, and subsequent worker mutations use the
same key and tenant binding. Canonical workers cannot use legacy claim routes.
For bearer-token
requests, `task_auth_label` must equal the label of the matching entry in
`A2A_AUTH_TOKENS`; policy-authenticated requests must carry the bound tenant.
Only the resulting hashed task ID is persisted as the generic idempotency
marker. Retries and concurrent workflow runners therefore converge on one task
across API replicas without deduplicating work across tenants. A new head SHA
or author slot produces a new task.

The conversation context excludes the head SHA, so successive reviews for the
same PR and author resume the same A2A conversation while retaining distinct,
idempotent work items.

## Workflow configuration

Each repository supplies deployment details rather than identity assumptions:

- `CODETETHER_RUNNER_LABEL`
- `CODETETHER_FORGEJO_API_URL` and the event-scoped Forgejo token
- `CODETETHER_API_URL` and `CODETETHER_TOKEN`; the latter must match a labeled
  entry in the server's `A2A_AUTH_TOKENS` when no policy user is attached
- `CODETETHER_REVIEW_TARGET_AGENT` for the independent reviewer
- `CODETETHER_REVIEW_MODEL`
- `CODETETHER_REVIEWER_CANDIDATES`, a comma-separated list of
  `forgejo-login=vault/path` entries

The workflow selects the first configured reviewer whose login differs from
the PR author. A configured review target is always submitted through
`/v1/agent/tasks`; the workflow refuses a JSON-only endpoint that cannot carry
the target identity. The Vault token is cleared after the review operation.

## Failure semantics

Failures are closed and visible:

- unverifiable signatures or inconsistent principals disable author delivery;
- missing server provenance-key configuration returns HTTP 503;
- invalid provenance or worker-possession proofs return HTTP 403 or 422;
- protocol-downgrade requests containing reusable-session metadata return
  HTTP 422;
- missing, invalid, or cross-principal CodeTether task credentials return HTTP
  401 or 403;
- missing server task-auth configuration returns HTTP 503;
- unavailable canonical workers return HTTP 409;
- unavailable durable storage or failed persistence returns HTTP 503;
- malformed protocol metadata returns HTTP 422;
- CodeTether task creation failure fails the workflow rather than claiming a
  successful author conversation.

The Forgejo review comment records the protocol version, canonical route,
CodeTether task ID, signed provenance ID, and immutable head SHA marker.