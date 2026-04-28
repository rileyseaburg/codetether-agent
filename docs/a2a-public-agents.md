# Public A2A Agents via the Agent Provenance Framework

How the zero-config peer-to-peer discovery design (mDNS-based, no central
broker) composes with the **Agent Provenance Framework** (APF) RFC —
[`Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt`](https://github.com/rileyseaburg/codetether/blob/main/rfc/Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt)
in the upstream `rileyseaburg/codetether` repo — to enable **public web
agents that can talk to each other** safely.

This document describes how `codetether spawn` / `codetether tui
--a2a-port <port>` peers extend from "two TUIs on a LAN" to "agents on
the open internet running tasks on each other's behalf, accountable
end-to-end."

---

## TL;DR

> A2A's wire format tells you **how** to call a peer.
> mDNS / DNS-SD tells you **where** the peer lives.
> The peer's signed agent card tells you **who** they are.
> An APF-claim-bearing token on every `message/send` tells you **what
> caused this action and on whose authority it acts** — enough to make a
> trust decision.
>
> Public A2A agents need all four.

---

## The four orthogonal layers

CodeTether's current peer-to-peer surface is a **transport + discovery**
substrate only. APF supplies the missing **identity + authorization**
layers needed before any of this is safe to expose to the open internet.

| Layer | Question it answers | Local LAN today | Public web requirement |
|---|---|---|---|
| **Transport** | How is the message encoded? | A2A JSON-RPC over HTTP, agent card at `/.well-known/agent.json`. ✅ in `src/a2a/server.rs`. | Same. Add TLS by default and DPoP-sender-constrained tokens (`cnf` claim — RFC §5.7). |
| **Discovery** | Where does the peer live? | Explicit `--peer` / `CODETETHER_A2A_PEERS` seeds today (see [`a2a-spawn.md`](a2a-spawn.md)). The mDNS-SD-on-`_codetether-a2a._tcp.local.` zero-config design is sketched in the same doc and being added in a follow-up PR. | Wide-area DNS-SD (RFC 6763 §11) **or** a federation registry. mDNS alone does not cross routers. |
| **Identity** | Who is the peer? | Card name string (no trust signal). | `AgentCard.signatures[]` JWS-signed by the peer's authorization server, verifiable against published JWKS. |
| **Authorization** | What is this caller authorized to cause here? | None — `handle_message_send` accepts any inbound. | Every `message/send` carries an APF-claim JWT with all five provenance dimensions verified per RFC §10. |

Each layer is **independent** of the others. mDNS doesn't replace identity;
identity doesn't replace authorization. The current implementation has
transport and (planned) discovery; APF supplies identity and
authorization.

---

## Why mDNS alone is not enough for the public web

The proposed mDNS-based zero-config flow — each agent advertises
`_codetether-a2a._tcp.local.`, peers browse and find each other — is
correct and sufficient **for trusted networks**:

- **Same host**: two `codetether tui` processes find each other instantly.
- **Same LAN**: agents in the same office/home find each other.

For the public web, mDNS provides **only locator information** — an IP
and a port. It carries no statement about *who* is at that IP, no
verifiable claim about what they will do with messages they receive, and
no statement of authorization for what they ask you to do.

A naive auto-intro flow (the current `send_intro` at
`src/a2a/spawn.rs`) sends a `message/send` to every newly-discovered
peer. On a public network this is:

- A **spam vector** outbound.
- A **command injection vector** inbound — unknown sender, no auth, runs
  a fresh `Session` and executes whatever the prompt says.
- A **prompt-injection amplifier**, since intro messages from untrusted
  agents become first-turn user content.

APF closes each of these gaps with a verifiable claim.

---

## Mapping APF's five dimensions onto an A2A peer call

When agent A (in trust domain `domain-a.example`) calls agent B (in
`domain-b.example`) over A2A, APF's five provenance dimensions appear
in the JWT carried on the `message/send` HTTP request. The receiving
end (B's resource server) verifies all five before invoking
`Session::new()`.

The mapping below assumes B is the **resource server / policy
enforcement point** for itself; in production deployments B's PEP can
be a sidecar (OPA reading the same Rego at [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego)).

### 1. Origin — `ap_origin` (RFC §5.2)

> "What was the user intent that started the session A is now extending
> into B?"

A's authorization server bound `ap_origin.intent_hash` at the start of
A's session. When A calls B, the **same** `ap_origin` rides along in the
cross-domain provenance token (CDPT, RFC §9). B verifies that the hash
is well-formed and consistent across A's prior turns; B does not
re-derive the hash, but B's policy MAY refuse intent classifications it
does not accept (e.g. B refuses to receive calls originated from
`urn:ap:intent:bulk-scrape`).

**Relevance to public agents**: prevents a compromised mid-session A
from "drifting" into a goal A's user never authorized, then weaponizing
B as the executor.

### 2. Inputs — `ap_inputs` (RFC §6)

> "What untrusted content has entered A's reasoning context, and does
> that taint conflict with what A is asking B to do?"

If A previously fetched a web page, that taint marker
(`urn:ap:taint:source:web-fetch`, "untrusted-external") sits in
`ap_inputs`. When A asks B to send email, B's policy checks
`taint_blocked_actions` (already implemented at
[`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego) and
[`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py)). `email:write` is blocked from
`untrusted-external` taint. B refuses.

**Relevance to public agents**: this is the LAN-equivalent of the
"lethal trifecta" defense (private data + untrusted content + external
exfil). Without it, every A2A peer is a confused-deputy hop ready to
launder a prompt-injection into a sanctioned-looking call.

**Wire change required**: today the A2A `Message` type
(`src/a2a/types.rs`) has a `metadata: HashMap<String, serde_json::Value>`.
Provenance claims travel either:

- in the `Authorization: DPoP <jwt>` header (preferred — keeps the
  JSON-RPC body unchanged), or
- under a reserved metadata key `apf:provenance` if header-level auth
  is unavailable.

Either way the token's body carries the `ap_*` claims; the on-the-wire
JSON-RPC envelope is unchanged.

### 3. Delegation — `ap_delegation` (RFC §7)

> "Whose authority is A acting under, and is what A is asking B to do
> within that authority?"

A's `ap_delegation.chain` records the principal hierarchy from A's user
all the way to A's current process, with **capability envelopes**
attenuating monotonically (RFC §7.2). When A calls B, B inspects:

- the chain for an attenuating `act` step that authorizes "speaking to
  domain-b.example agents" with the requested intent,
- the leaf envelope's `operations` list against the action B is being
  asked to perform,
- the `spawn.max_depth` value — if A's chain has `max_depth: 0`, A
  cannot spawn further sub-agents on B's side either.

The reference policy in [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego)
already implements this: `_attenuation_failures` (Python at
[`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py))
walks adjacent pairs and checks operations are subsets, constraints
monotonically tighten, budgets do not exceed parent.

**Relevance to public agents**: prevents A from calling B with a
capability it never had — the classic "I'm a trusted agent, please
delete this database" attack stops at the verifier.

### 4. Runtime — `ap_runtime` (RFC §8.1)

> "What execution environment produced the model output that emitted
> this peer call?"

A signed runtime attestation: model id (e.g. `claude-opus-4-7`),
hashed system prompt, hashed tool manifest, hashed config,
attestation type and signature. B verifies the signature against the
attestation authority listed in A's archetype metadata.

For the current CodeTether codebase, runtime attestation is currently
"service-level" — i.e. trust the model provider's TLS endpoint as the
attestation. Hardware attestation (Nitro Enclaves, SEV-SNP, TDX) is a
deployment-specific upgrade path. The verifier already accepts
"presence of `ap_runtime`" as the runtime dimension being satisfied;
strict policies will demand a non-`service-level` attestation type.

**Relevance to public agents**: prevents an attacker who steals A's
peer credentials from minting `message/send` calls without ever running
the model — the attestation quote can only be produced by the attested
runtime.

### 5. Output — `ap_output` (RFC §8.2)

> "Is this specific peer call the actual output of the model run whose
> attestation A is claiming?"

The output binding includes `tool_call_hash` over the canonicalized
peer-call JSON. B recomputes the hash from the JSON-RPC body it
received and compares. If the orchestrator on A's side has been
compromised but the model has not, fabricated calls fail this check.

**Relevance to public agents**: closes the "compromised orchestrator"
hole — even if A's host machine is rooted, only calls the model
genuinely emitted will pass B's gate.

---

## Cross-domain peer hop, end-to-end

Concrete walk-through for a public-internet A2A call:

1. **Discovery**: A's auto-discovery (mDNS for LAN, federation registry
   for wide area — see "Wide-area discovery" below) returns
   `https://b.domain-b.example/`.
2. **Card fetch**: A `GET /.well-known/agent.json` over TLS. The card's
   `signatures[]` field (currently empty in
   `src/a2a/types.rs`) carries one or more JWS over the JCS
   (RFC 8785)-canonicalized card body. A verifies against the
   published JWKS at the issuer (typically
   `<issuer>/.well-known/jwks.json`).
3. **Card-level policy check**: A's policy may reject based on
   `provider`, `capabilities`, `securitySchemes`, `security`
   requirements (e.g. "I only call peers that require DPoP-bound
   tokens").
4. **Token mint**: A's authorization server performs an extended RFC
   8693 token exchange (`requested_token_type:
   urn:ietf:params:oauth:token-type:agent-provenance-jwt`) producing a
   **CDPT** (RFC §9) with:
   - A's `ap_origin` copied verbatim,
   - A's current `ap_inputs` (taints accumulated so far),
   - A's `ap_delegation` extended with one step recording "A is calling
     B in domain-b.example with attenuated envelope X",
   - `ap_runtime` from A's current inference quote,
   - `ap_output` over the JSON-RPC body A is about to send.
5. **Wire**: A POSTs the JSON-RPC envelope to B's `/` with
   `Authorization: DPoP <CDPT>`. The DPoP proof is bound via `cnf`
   (RFC §5.7).
6. **Verification at B** (per RFC §10):
   - Step 1: token integrity, expiry, audience, DPoP proof.
   - Step 2: `ap_origin` consistency.
   - Step 3: `ap_inputs` taint completeness.
   - Step 4: `ap_delegation` attenuation walk.
   - Step 5: `ap_runtime` attestation signature against B's accepted
     authorities (which may differ from A's domain).
   - Step 6: `ap_output.tool_call_hash` recomputed over the body B
     received.
   - Step 7: B's policy applied.
7. If authorized: B's AS issues a domain-b-local token, B's resource
   server invokes `Session::new()` for the inbound call, emits its
   reply with B's own `ap_*` claims, and returns over the same JSON-RPC
   response.
8. **Decision log** (RFC §10.3): both A and B persist the verification
   result with all dimensions, supporting later audit reconstruction.
   The reference impl emits `ProvenanceDecision.as_dict()`
   ([`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py)).

---

## Wide-area discovery — replacing/augmenting mDNS

mDNS (`_codetether-a2a._tcp.local.`) only crosses LAN segments. For
public agents, three options compose with the framework:

### A. Wide-area DNS-SD (RFC 6763 §11)

Same record format as mDNS, but published in a real DNS zone:

```
_codetether-a2a._tcp.example.com   PTR     riley-research.example.com.
riley-research._codetether-a2a._tcp.example.com  SRV  10 10 443 a.example.com.
                                                 TXT  "name=riley-research" "path=/" "protocol=a2a-jsonrpc"
```

Pros: zero new infrastructure beyond DNS hosting. Cons: requires
controlling a DNS zone per peer.

### B. Federation registry

A trusted registry endpoint at e.g. `https://registry.codetether.run/`
publishes a feed of registered peers. Each registered card is
JWS-signed by the registry; consuming agents pin the registry's JWKS.
This is the natural extension of `bus.registry.register(card)` already
present in `src/bus/registry.rs:25`, lifted from in-process to cross-host.

This composes with APF: the registry doesn't authorize action
requests — APF tokens still do. The registry is just discovery.

### C. Out-of-band exchange (URL paste, QR code, email)

The simplest case: A's user emails A's signed card to B's user. B
imports it. No protocol-level discovery at all.

In all three cases, **the agent card's JWS signatures are what create
trust** — discovery is just a name service. APF then authorizes
individual calls on top.

---

## Required schema additions to compose APF with A2A

These are the concrete deltas the current code at `src/a2a/` needs to
become APF-ready. They are **declarative changes to the wire format**
not new transports.

### 1. AgentCard signatures populated

Field already exists at `src/a2a/types.rs` (`AgentCardSignature`),
but `default_card` (`src/a2a/server.rs`) sets it to `vec![]`. For
public agents, every card MUST be JWS-signed by the AS that minted it.

```rust
// What it should look like for public agents:
signatures: vec![AgentCardSignature {
    signature: "<JWS compact>",
    algorithm: Some("ES256".to_string()),
    key_id: Some("https://as.domain-a.example/keys/rotation-2026q2".to_string()),
}],
```

### 2. AgentCard security requirements explicit

`AgentCard.security_schemes` and `security` (`src/a2a/types.rs`)
currently default to empty. For public agents:

```rust
security_schemes.insert(
    "dpop".to_string(),
    SecurityScheme::Http {
        scheme: "dpop".to_string(),
        bearer_format: Some("apf-jwt".to_string()),
        description: Some("Agent Provenance Framework JWT, DPoP-bound".into()),
    },
);
security = vec![hashmap!{ "dpop" => vec![] }];
```

This declares: "I require DPoP-bound APF tokens. Bearer rejected."
A2A clients calling this card already know what to mint before sending.

### 3. JSON-RPC server: provenance verification gate

`handle_message_send` at `src/a2a/server.rs` currently runs:

```
extract prompt → Session::new() → session.prompt() → reply
```

For public peers, insert RFC §10 between extract-prompt and
Session::new():

```
extract prompt
  → parse Authorization: DPoP <jwt>
  → call provenance::verify_provenance(claims, action="agent:execute", resource={session-state})
  → if !decision.allowed_by_provenance: return JsonRpcError::with_code(-32007, ...)
  → if decision.partial && policy.refuses_partial: return ...
  → Session::new()
  → ...
```

The verification function already exists in the Python sidecar at
[`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py) and Rego at
[`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego). The Rust server can either:

- **A**: call out to OPA over HTTP for each inbound (deployment-friendly,
  policy hot-reloadable), or
- **B**: link a Rust verifier mirroring the Python module (lower
  latency, requires keeping two impls in sync).

For initial public-agent rollout, option A (OPA sidecar) is simpler.

### 4. Outbound: A2AClient stamps tokens before sending

`src/a2a/client.rs` currently sends a JSON-RPC body with no auth
beyond an optional `CODETETHER_AUTH_TOKEN` bearer. For public peers it
must:

- Mint a CDPT via the local AS's token-exchange endpoint.
- Compute `ap_output.tool_call_hash` over the canonicalized JSON-RPC
  body it is about to send.
- Compute the DPoP proof.
- Attach `Authorization: DPoP <CDPT>` and `DPoP: <proof>` headers.

`A2AClient::send_message` becomes a thin wrapper over a new
`A2AClient::send_provenance_message` that takes a `ProvenanceContext`
(session id, current taints, current attestation quote). The `with_token`
method (`src/a2a/client.rs`) is repurposed for legacy peers only.

### 5. Auto-intro becomes provenance-aware

The current `send_intro` (`src/a2a/spawn.rs`) blindly POSTs to every
newly-discovered peer. For public agents:

- The intro carries an APF token like any other call.
- `ap_origin.intent_classification` is set to a reserved
  `urn:ap:intent:peer-introduction` (per RFC §11 URN namespace).
- Receiving policy classifies intros leniently (no real action being
  requested) but logs all unknown-sender intros for review.
- Auto-intro is OPT-IN for public peers, default-on only for LAN.

---

## Threat model shift: LAN → public web

| Concern | LAN (today) | Public web (with APF) |
|---|---|---|
| Identity | Trust the LAN. | Card JWS verified against AS JWKS. |
| Tampering in transit | TLS optional. | TLS mandatory. DPoP sender-constrains the token to a public key via `cnf` (RFC 7800); each request carries a DPoP proof signed by the corresponding private key. |
| Replay | Not addressed. | DPoP `nonce` + APF attestation freshness window (RFC §8.4). |
| Confused deputy / capability escalation | Trust the caller. | `ap_delegation` attenuation walk (RFC §7.2 + Rego at [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego)). |
| Prompt injection laundering across peers | Caller is trusted; no taint propagated. | `ap_inputs` propagated cross-domain in CDPT (RFC §6.3, §9). |
| Compromised orchestrator forging calls | Not addressed. | `ap_runtime` + `ap_output` (RFC §8). |
| Fan-out DoS via auto-spawn | Local `--discovery-interval-secs` cap. | Per-token `spawn.max_depth` and `max_fanout` envelope (RFC §7.2). |
| Cross-domain audit | Logs are local. | Decision log per RFC §10.3 produces a reconstructable trace; CDPT records the full chain. |

The concerns on the right are **the cost of going public**. APF is
the framework that makes paying that cost feasible; mDNS-based
auto-discovery without APF would just expand the attack surface.

---

## Composition with the existing reference implementation

The Python provenance verifier and Rego policy in `../` already cover
the *receiving* side of a public A2A call:

| File | Role | Use as-is for public agents? |
|---|---|---|
| [`rfc/Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt`](https://github.com/rileyseaburg/codetether/blob/main/rfc/Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt) | Normative spec | ✅ |
| [`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py) | Per-action verifier (origin, inputs, delegation; runtime/output presence) | ✅ for receiving end. Needs Rust port if avoiding the OPA sidecar route. |
| [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego) | OPA policy mirror of the Python verifier | ✅ |
| [`a2a_server/policy.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/policy.py) | Glue: combines provenance verifier with action authorization | ✅ |
| `src/a2a/server.rs` (this repo) | A2A inbound JSON-RPC handler | **Needs hook** to call the provenance verifier before `Session::new()`. |
| `src/a2a/client.rs` (this repo) | A2A outbound caller | **Needs CDPT minting** and DPoP signing. |
| `src/a2a/types.rs` (this repo) | AgentCard + JSON-RPC types | Schema is already APF-compatible (signatures, security_schemes, security). Just needs to be populated. |
| `src/a2a/mdns.rs` (deferred) | mDNS discovery | LAN-only by design. Wide-area uses DNS-SD or federation registry. |

The lift to make CodeTether agents APF-compliant on the public web is
**a hook in the Rust A2A handler that calls the existing OPA policy
sidecar**, plus **a token-exchange step in the outbound client**. The
heavy lifting (the policy) is already shipped in `../`.

---

## Roadmap (this is the documented design, not the current state)

1. **(Today)** A2A peer transport ships in this repo. PR #111 adds the
   TUI A2A surface. Auto-pick + mDNS designed in [a2a-spawn.md](a2a-spawn.md),
   not yet built.
2. **Next**: implement OPA-sidecar provenance gate in
   `handle_message_send`. Inbound calls without APF claims continue to
   be accepted on loopback; rejected on non-loopback by default.
3. **Then**: implement CDPT minting in `A2AClient` via the local AS's
   token-exchange endpoint. `CODETETHER_AUTH_TOKEN` bearer becomes a
   legacy fallback flag.
4. **Then**: card signing pipeline. `default_card` becomes
   `default_card_signed_by(as_jwks_url)`; the AS endpoint signs and
   rotates.
5. **Then**: federation registry and/or wide-area DNS-SD for cross-LAN
   discovery, replacing mDNS for public deployments.
6. **Then**: hardware runtime attestation upgrade path
   (TDX / Nitro / SEV-SNP), gated by archetype metadata.
7. **Then**: revocation propagation (RFC §7.4, §9.3) so when a chain
   parent is revoked all derived sessions terminate within 60s.

Each step composes with the prior; nothing built today is wasted.

---

## What this means in practice

- **Two TUIs on a laptop talking to each other** — works zero-config
  with the existing repo state plus the planned auto-pick + mDNS work.
  No APF needed; trust is "I own both processes."
- **Two TUIs across an office network** — same. Bind to `0.0.0.0`,
  mDNS over the LAN. Trust is "I own the LAN."
- **Two CodeTether agents in different organizations** — needs APF
  end-to-end. Discovery via federation registry or wide-area DNS-SD;
  identity via signed cards; authorization via CDPTs verified against
  the policy in [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego). **This is where the
  framework earns its keep.**
- **Public agent serving any caller** — same as the cross-org case
  plus stricter policy on `intent_classification`, attestation type,
  and per-caller rate limits.

---

## Open questions / non-goals

- **Encryption at rest of session state** — out of scope for APF;
  required for compliance deployments. Use existing Vault integration
  in `src/secrets/`.
- **Privacy-preserving discovery** (peers find each other without
  revealing existence to a registry) — not addressed. Federation
  registry is a known plaintext rendezvous.
- **Sybil resistance in the federation registry** — depends on the
  registry's onboarding policy (KYC, payment gating, etc). RFC is
  policy-neutral; deployments choose.
- **Routing** — APF doesn't say which peer to talk to. That's a
  separate "agent yellow pages" problem.

---

## References

- **RFC**: [`rfc/Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt`](https://github.com/rileyseaburg/codetether/blob/main/rfc/Agent-Provenance-Framework_for_Autonomous-Multi-Agent-Systems.txt)
  (`draft-seaburg-agent-provenance-00`).
- **Reference verifier (Python)**: [`a2a_server/provenance.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/provenance.py).
- **Reference policy (OPA/Rego)**: [`policies/provenance.rego`](https://github.com/rileyseaburg/codetether/blob/main/policies/provenance.rego).
- **Glue**: [`a2a_server/policy.py`](https://github.com/rileyseaburg/codetether/blob/main/a2a_server/policy.py).
- **A2A protocol**: in-tree protobuf at `proto/a2a/v1/a2a.proto`; upstream
  JSON spec [`specification/json/a2a.json`](https://github.com/rileyseaburg/codetether/blob/main/specification/json/a2a.json) is what this repo's wire format tracks.
- **CodeTether A2A surfaces**:
  - [docs/a2a-spawn.md](a2a-spawn.md) — current peer transport + the
    deferred mDNS / auto-pick zero-config design.
  - `src/a2a/server.rs` — inbound JSON-RPC handler (the
    provenance-verification hook lands here).
  - `src/a2a/client.rs` — outbound caller (CDPT minting lands here).
  - `src/a2a/types.rs` — wire types (already APF-compatible).
- **External**: RFC 6763 (DNS-SD), RFC 8693 (Token Exchange), RFC 8785
  (JCS), RFC 9449 (DPoP), RFC 7800 (`cnf`).
