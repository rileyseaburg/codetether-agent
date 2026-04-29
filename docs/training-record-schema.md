# Training Record Schema Reference

## Overview

CodeTether's S3 bus sink persists all agent bus messages as JSONL files to
MinIO/S3, structured as OpenAI chat-completions records for LLM fine-tuning
(SFT, DPO, RLHF).

## S3 Key Layout

```
{prefix}/v2/{YYYY}/{MM}/{DD}/{HH}/batch_{timestamp}_{uuid}.jsonl
```

Example: `training/v2/2026/04/27/20/batch_20260427T201709_a9ac1f27.jsonl`

## Schema Version: `v2.1`

The current schema version is `v2.1`. Every record includes
`metadata.schema_version` so consumers can handle mixed-schema batches.

## Record Shape

Each line is a JSON object following the OpenAI chat-completions format:

```json
{
  "role": "system | user | assistant | tool",
  "content": "...",
  "tool_calls": [...],
  "tool_call_id": "...",
  "name": "...",
  "metadata": {
    "schema_version": "v2.1",
    "bus_kind": "tool_request_batch | tool_request | tool_response | ...",
    "envelope_id": "uuid",
    "correlation_id": "uuid",
    "timestamp": "RFC3339",
    "topic": "agent.{name}.tool.{request,response}",
    "sender_id": "agent-name",
    "step": 7,
    "model": "gpt-5.5-fast",
    "provider": "openai-codex",
    "model_backend": "chatgpt-codex-responses-ws",
    "usage": {
      "input_tokens": 47975,
      "output_tokens": 257,
      "latency_ms": 8094
    }
  }
}
```

## Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | string | Schema version (e.g. `v2.1`) |
| `bus_kind` | string | BusMessage variant name (snake_case) |
| `envelope_id` | string | Unique envelope UUID |
| `timestamp` | string | ISO 8601 timestamp |
| `topic` | string | Hierarchical routing topic |
| `sender_id` | string | Originating agent name |
| `correlation_id` | string? | Links request → response |
| `step` | int? | Agent loop step number |
| `model` | string? | LLM model identifier (v2.1+) |
| `provider` | string? | Provider name (v2.1+) |
| `model_backend` | string? | Backend transport (v2.1+) |
| `usage` | object? | Token usage metrics (v2.1+) |

### Usage Object (v2.1+)

Only present on assistant-role records where the provider reports usage.

| Field | Type | Description |
|-------|------|-------------|
| `input_tokens` | uint64 | Prompt tokens consumed |
| `output_tokens` | uint64 | Completion tokens produced |
| `latency_ms` | uint64 | Wall-clock latency |

## Bus Kind Values

| Kind | Role | Description |
|------|------|-------------|
| `agent_ready` | system | Agent came online |
| `agent_shutdown` | system | Agent shutting down |
| `agent_message` | assistant | Inter-agent message |
| `task_update` | system | Task status change |
| `artifact_update` | system | Artifact produced |
| `shared_result` | system | Shared result published |
| `tool_request_batch` | assistant | Batched tool calls |
| `tool_request` | assistant | Single tool call |
| `tool_response` | tool | Tool execution result |
| `tool_output_full` | tool | Full untruncated output |
| `agent_thinking` | assistant | Chain-of-thought reasoning |
| `ralph_learning` | system | Ralph iteration learnings |
| `ralph_handoff` | system | Ralph story handoff |
| `ralph_progress` | system | Ralph PRD progress |
| `voice_session_started` | system | Voice session created |
| `voice_transcript` | user/assistant | Voice transcript fragment |
| `voice_agent_state_changed` | system | Voice agent state change |
| `voice_session_ended` | system | Voice session ended |

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `MINIO_ENDPOINT` | required | S3 endpoint URL |
| `MINIO_ACCESS_KEY` | required | Access key |
| `MINIO_SECRET_KEY` | required | Secret key |
| `CODETETHER_BUS_S3_BUCKET` | `codetether-training` | Bucket name |
| `CODETETHER_BUS_S3_PREFIX` | `training/` | Path prefix |
| `CODETETHER_BUS_S3_BATCH_SIZE` | `100` | Records per flush |
| `CODETETHER_BUS_S3_FLUSH_SECS` | `30` | Flush interval |
| `CODETETHER_BUS_S3_INCLUDE_THINKING` | `true` | Include agent_thinking records |

### Thinking Records

`agent_thinking` records contain raw chain-of-thought reasoning from the
model. These are critical for reasoning distillation (SFT into small models).

- **Default**: enabled (`true`)
- **Disable**: Set `CODETETHER_BUS_S3_INCLUDE_THINKING=false` for sensitive
  workloads where chain-of-thought must not be persisted to S3.

## Migration from v1

| Feature | v1 | v2.0 | v2.1 |
|---------|----|------|------|
| S3 key prefix | `training/{YYYY}/` | `training/v2/{YYYY}/` | `training/v2/{YYYY}/` |
| `agent_thinking` records | ✅ | ❌ (regression) | ✅ (config-gated) |
| `model` attribution | ❌ | ❌ | ✅ |
| `provider` attribution | ❌ | ❌ | ✅ |
| `usage` metrics | ❌ | ❌ | ✅ |
| `schema_version` | ❌ | ❌ | ✅ |
