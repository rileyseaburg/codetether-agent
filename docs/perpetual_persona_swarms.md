# Perpetual Persona Swarms (Phase 0)

This document describes the initial `codetether-agent` implementation for
always-on cognition and persona lifecycle management.

## Scope

Phase 0 includes:

- Contract schemas for personas, thought events, proposals, and memory snapshots
- In-memory runtime manager with bounded buffers
- Feature-flagged perpetual loop (`observe -> reflect -> test -> compress`)
- Model-backed thinker integration via OpenAI-compatible chat completions
- Server endpoints for cognition control and swarm persona lifecycle

Phase 0 does **not** include external side-effect execution from cognition output.

## Feature Flags

Set these when running `codetether serve`:

- `CODETETHER_COGNITION_ENABLED=true`
- `CODETETHER_COGNITION_AUTO_START=true`
- `CODETETHER_COGNITION_LOOP_INTERVAL_MS=2000`
- `CODETETHER_COGNITION_MAX_SPAWN_DEPTH=4`
- `CODETETHER_COGNITION_MAX_BRANCHING_FACTOR=4`
- `CODETETHER_COGNITION_MAX_EVENTS=2000`
- `CODETETHER_COGNITION_MAX_SNAPSHOTS=128`
- `CODETETHER_COGNITION_THINKER_ENABLED=true`
- `CODETETHER_COGNITION_THINKER_BACKEND=openai_compat` (`candle` for in-process inference)
- `CODETETHER_COGNITION_THINKER_BASE_URL=http://127.0.0.1:11434/v1`
- `CODETETHER_COGNITION_THINKER_MODEL=qwen2.5:3b-instruct`
- `CODETETHER_COGNITION_THINKER_API_KEY=` (optional)
- `CODETETHER_COGNITION_THINKER_TEMPERATURE=0.2`
- `CODETETHER_COGNITION_THINKER_MAX_TOKENS=256`
- `CODETETHER_COGNITION_THINKER_TIMEOUT_MS=12000`
- `CODETETHER_COGNITION_THINKER_CANDLE_MODEL_PATH=/models/model.gguf` (required for `candle`)
- `CODETETHER_COGNITION_THINKER_CANDLE_TOKENIZER_PATH=/models/tokenizer.json` (required for `candle`)
- `CODETETHER_COGNITION_THINKER_CANDLE_ARCH=qwen2` (`llama` and `qwen2` supported)
- `CODETETHER_COGNITION_THINKER_CANDLE_DEVICE=auto` (`auto`, `cpu`, `cuda`)
- `CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL=0`

If no persona exists at start time, cognition auto-seeds a default `root-thinker` persona unless `seed_persona` is supplied in `POST /v1/cognition/start`.
CUDA execution requires building with `--features candle-cuda` (or `candle-cudnn`).

Convenience targets in `codetether-agent/Makefile`:
- `make build-cuda`
- `make deploy-spike2-cuda`
- `make status-spike2-cuda`

## Endpoints

### Cognition Control

- `POST /v1/cognition/start`
- `POST /v1/cognition/stop`
- `GET /v1/cognition/status`
- `GET /v1/cognition/stream` (SSE)
- `GET /v1/cognition/snapshots/latest`

### Swarm Persona Lifecycle

- `POST /v1/swarm/personas`
- `POST /v1/swarm/personas/{id}/spawn`
- `POST /v1/swarm/personas/{id}/reap`
- `GET /v1/swarm/lineage`

## Contracts

Implemented in `src/cognition/mod.rs`:

- `PersonaIdentity`
- `PersonaPolicy`
- `PersonaRuntimeState`
- `ThoughtEvent` + `ThoughtEventType`
- `Proposal` + `ProposalStatus` + `ProposalRisk`
- `MemorySnapshot`
- `LineageGraph` + `LineageNode`

## Example Requests

Start cognition loop:

```json
{
  "loop_interval_ms": 750,
  "seed_persona": {
    "name": "root-orchestrator",
    "role": "orchestrator",
    "charter": "Monitor, synthesize, and coordinate",
    "swarm_id": "swarm-alpha"
  }
}
```

Create root persona:

```json
{
  "name": "persona-alpha",
  "role": "architect",
  "charter": "Maintain architecture coherence",
  "swarm_id": "swarm-alpha"
}
```

Spawn child persona:

```json
{
  "name": "persona-alpha-reviewer",
  "role": "reviewer",
  "charter": "Challenge assumptions and verify quality"
}
```

Reap with cascade:

```json
{
  "cascade": true,
  "reason": "resource_budget_reclaim"
}
```

## Safety Notes

- Persona recursion is bounded by depth and branching limits.
- Cognition loop emits proposals/events only.
- No unrestricted external actuation path is included in this phase.
- When thinker inference fails, runtime falls back to deterministic thought payloads and records the error in event metadata.
