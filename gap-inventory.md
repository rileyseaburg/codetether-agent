# Gap Inventory: A2A Ecosystem Worker Integration

**Project:** codetether-agent  
**Feature:** A2A Ecosystem Worker Integration  
**Branch:** feature/a2a-worker-integration  
**Version:** 1.0  
**Generated:** 2026-02-06

---

## Executive Summary

This document provides a comprehensive inventory of 7 identified gaps in the A2A (Agent-to-Agent) Ecosystem Worker Integration feature. Total estimated effort: **19 story points** (approximately 3-4 weeks for a single developer, or 1-2 weeks for a team of 2-3).

---

## 1. Gap Inventory Overview

| ID | Title | Priority | Complexity | Status | Dependencies |
|----|-------|----------|------------|--------|--------------|
| US-001 | Wire Session Execution in A2A Worker | P1 | 2 | 🔴 Open | None |
| US-002 | Stream Task Output to A2A Server | P1 | 3 | 🔴 Open | US-001 |
| US-003 | Implement Worker Heartbeat Loop | P2 | 2 | 🔴 Open | None |
| US-004 | Wire A2A Server message/send to Session Execution | P2 | 3 | 🔴 Open | US-001 |
| US-005 | Implement A2A Server message/stream Endpoint | P3 | 4 | 🔴 Open | US-004 |
| US-006 | Report Available Models to A2A Server on Registration | P3 | 3 | 🔴 Open | US-003 |
| US-007 | Task Status Updates During Execution | P2 | 2 | 🔴 Open | US-001, US-002 |

**Legend:**
- Priority: P1 (Critical), P2 (High), P3 (Medium)
- Complexity: Story points (1-4 scale)
- Status: 🔴 Open, 🟡 In Progress, 🟢 Complete

---

## 2. Detailed Gap Analysis

### Gap US-001: Wire Session Execution in A2A Worker
**Priority:** 🔴 Critical (P1)  
**Complexity:** 2 story points (~2-3 days)  
**Dependencies:** None

#### Current State
The `handle_task()` function in `src/a2a/worker.rs` contains a TODO stub that only logs a message instead of executing the session.

#### Technical Requirements
- Replace TODO stub with actual `session.prompt_with_events()` call
- Handle `AutoApprove` policy configuration:
  - `AutoApprove::All` → automatic tool approval
  - `AutoApprove::Safe` → auto-approve read-only tools only
  - `AutoApprove::None` → reject tool calls requiring approval
- Error handling: catch errors and release task with status 'failed'

#### Acceptance Criteria
1. ✅ `handle_task()` calls `session.prompt_with_events()` instead of logging stub
2. ✅ `SessionResult.text` sent as task result in release payload
3. ✅ `AutoApprove::All` configures automatic tool approval
4. ✅ `AutoApprove::Safe` auto-approves read-only tools only
5. ✅ `AutoApprove::None` rejects approval-required tool calls
6. ✅ Errors caught and task released with 'failed' status + error message
7. ✅ `cargo check` passes with no new warnings

#### Prerequisites
- Session module must be functional
- A2A worker HTTP client must be configured
- Tool approval system must be implemented

#### Risk Factors
- Low risk - foundational feature with clear implementation path

---

### Gap US-002: Stream Task Output to A2A Server
**Priority:** 🔴 Critical (P1)  
**Complexity:** 3 story points (~3-4 days)  
**Dependencies:** US-001

#### Current State
No real-time output streaming exists; task results are only sent at completion.

#### Technical Requirements
- Spawn tokio task to consume `mpsc::Receiver<SessionEvent>`
- POST output chunks to `/v1/opencode/tasks/{id}/output`
- Handle event types:
  - `SessionEvent::TextChunk` → accumulated text
  - `SessionEvent::ToolCallStart` → structured tool running indicator
  - `SessionEvent::ToolCallComplete` → tool output
  - `SessionEvent::Error` → error information
- Non-blocking implementation
- Graceful degradation if server unreachable

#### Acceptance Criteria
1. ✅ Background tokio task consumes `SessionEvent` from channel
2. ✅ `TextChunk` sends accumulated text to output endpoint
3. ✅ `ToolCallStart` sends structured tool-running update
4. ✅ `ToolCallComplete` sends tool output to server
5. ✅ `Error` sends error information to server
6. ✅ Output streaming doesn't block main session execution
7. ✅ Server unreachable → warning logged, execution continues
8. ✅ Bearer token auth included in POST requests when configured

#### Prerequisites
- US-001 completion (session execution)
- A2A server output endpoint available
- `SessionEvent` channel implementation

#### Risk Factors
- Medium risk - requires careful async handling to avoid blocking

---

### Gap US-003: Implement Worker Heartbeat Loop
**Priority:** 🟡 High (P2)  
**Complexity:** 2 story points (~2-3 days)  
**Dependencies:** None

#### Current State
No heartbeat mechanism exists to keep worker registered as alive.

#### Technical Requirements
- Periodic heartbeat every 30 seconds
- POST to `/v1/opencode/workers/{worker_id}/heartbeat`
- Background tokio task alongside SSE stream
- Include status: idle/processing, active_task_count
- Failure resilience: 3 consecutive failures → log error, don't crash
- Cancellation/restart on SSE reconnection

#### Acceptance Criteria
1. ✅ Heartbeat POST every 30 seconds
2. ✅ Payload includes worker_id, agent_name, status, active_task_count
3. ✅ Heartbeat starts on connect, stops on shutdown
4. ✅ 3 consecutive failures log error but don't terminate worker
5. ✅ Uses same auth token as SSE connection
6. ✅ Heartbeat loop cancelled/restarted on SSE reconnection

#### Prerequisites
- A2A worker HTTP client configured
- SSE connection management in place

#### Risk Factors
- Low risk - standard background task pattern

---

### Gap US-004: Wire A2A Server message/send to Session Execution
**Priority:** 🟡 High (P2)  
**Complexity:** 3 story points (~3-4 days)  
**Dependencies:** US-001

#### Current State
`handle_message_send()` in `src/a2a/server.rs` is not implemented for actual task processing.

#### Technical Requirements
- Spawn background task for async message processing
- Create Session and execute prompt
- Task state transitions: Submitted → Working → Completed/Failed
- Store result as Task Artifact (text/plain)
- Update Task in DashMap in-place
- Support concurrent parallel task execution

#### Acceptance Criteria
1. ✅ `handle_message_send()` spawns tokio task for async processing
2. ✅ Task state transitions properly tracked
3. ✅ Task status.timestamp updated at each transition
4. ✅ Session result stored as Artifact with type text/plain
5. ✅ Task updated in DashMap for `tasks/get` reflection
6. ✅ Concurrent calls create independent parallel tasks
7. ✅ Provider/model configurable via server state or defaults

#### Prerequisites
- US-001 completion (session execution pattern)
- A2A server infrastructure in place
- DashMap for task storage

#### Risk Factors
- Medium risk - requires proper async task management

---

### Gap US-005: Implement A2A Server message/stream Endpoint
**Priority:** 🟢 Medium (P3)  
**Complexity:** 4 story points (~4-5 days)  
**Dependencies:** US-004

#### Current State
`handle_message_stream()` returns an error instead of an SSE stream.

#### Technical Requirements
- Return axum `Sse` response type
- Stream `SessionEvent` variants as JSON-encoded SSE events
- Event types: task_status, text_chunk, tool_call (start/complete), done
- Proper stream closure on terminal states
- Client disconnection cleanup

#### Acceptance Criteria
1. ✅ Returns axum Sse response instead of error
2. ✅ SSE events: task_status, text_chunk, tool_call, done
3. ✅ Each event is JSON with 'type' and 'data' fields
4. ✅ Stream closes after terminal state reached
5. ✅ Client disconnection doesn't leave orphaned tasks
6. ✅ AgentCapabilities.streaming already true in default_card()

#### Prerequisites
- US-004 completion (server-side session execution)
- axum SSE support
- Session event streaming infrastructure

#### Risk Factors
- Higher complexity - SSE streaming requires careful resource management

---

### Gap US-006: Report Available Models to A2A Server on Registration
**Priority:** 🟢 Medium (P3)  
**Complexity:** 3 story points (~3-4 days)  
**Dependencies:** US-003

#### Current State
Worker registration doesn't report available models or capabilities.

#### Technical Requirements
- Query ProviderRegistry during `register_worker()`
- Call `list_models()` on each provider
- Include models grouped by provider in registration payload
- Include capabilities: ralph, swarm, rlm, a2a, mcp
- Graceful handling if Vault unreachable
- Re-report on SSE reconnection

#### Acceptance Criteria
1. ✅ `register_worker()` loads ProviderRegistry and calls `list_models()`
2. ✅ Registration payload includes 'models' field (grouped by provider)
3. ✅ Registration payload includes 'capabilities' field
4. ✅ Vault unreachable → proceed without model info, log warning
5. ✅ Models re-reported on each SSE reconnection

#### Prerequisites
- US-003 completion (heartbeat/registration infrastructure)
- ProviderRegistry functional
- Vault integration for secrets

#### Risk Factors
- Medium risk - requires ProviderRegistry integration

---

### Gap US-007: Task Status Updates During Execution
**Priority:** 🟡 High (P2)  
**Complexity:** 2 story points (~2-3 days)  
**Dependencies:** US-001, US-002

#### Current State
No real-time status updates during task execution.

#### Technical Requirements
- PUT to `/v1/opencode/tasks/{id}/status` for status transitions
- Track stages: claimed → working → streaming → completed/failed
- Include metadata: provider name, model, tool_calls_count
- Include token usage on completion
- Include error message on failure

#### Acceptance Criteria
1. ✅ After claiming, status updated to 'working'
2. ✅ Status payload includes provider name and model
3. ✅ Status payload includes tool_calls_count
4. ✅ Final status includes total tokens used
5. ✅ Failure status includes error message + partial output
6. ✅ Uses same auth token and error handling as other calls

#### Prerequisites
- US-001 completion (session execution)
- US-002 completion (output streaming infrastructure)
- A2A server status endpoint available

#### Risk Factors
- Low risk - builds on existing infrastructure

---

## 3. Dependency Graph

```
US-001 (Wire Session Execution)
├── US-002 (Stream Output) ──┬── US-007 (Status Updates)
│                            │
└── US-004 (Server message/send)
    └── US-005 (Server message/stream)

US-003 (Heartbeat)
└── US-006 (Report Models)
```

### Critical Path Analysis
**Path 1 (Worker-side):** US-001 → US-002 → US-007 (7 points, ~1.5 weeks)  
**Path 2 (Server-side):** US-001 → US-004 → US-005 (9 points, ~2 weeks)  
**Path 3 (Registration):** US-003 → US-006 (5 points, ~1 week)

**Critical Path:** US-001 → US-004 → US-005 (longest at 9 points)

---

## 4. Technical Requirements (Global)

All gaps must adhere to these technical requirements:

| # | Requirement | Impact |
|---|-------------|--------|
| 1 | Pass `cargo check`, `cargo clippy --all-features`, `cargo test` | All gaps |
| 2 | Use `tracing` for logging (info/warn/error with structured fields) | All gaps |
| 3 | API keys from Vault only — never hardcode or use env vars | All gaps |
| 4 | HTTP requests include bearer auth when token configured | All gaps |
| 5 | Background tasks cancellable via `CancellationToken` or `JoinHandle::abort` | US-002, US-003 |
| 6 | Error handling: use `anyhow::Result`, no panics, log gracefully | All gaps |

---

## 5. Effort Estimation Summary

### By Priority

| Priority | Count | Total Points | Estimated Effort |
|----------|-------|--------------|------------------|
| P1 (Critical) | 2 | 5 | 5-7 days |
| P2 (High) | 3 | 7 | 7-10 days |
| P3 (Medium) | 2 | 7 | 7-10 days |
| **Total** | **7** | **19** | **19-27 days** |

### By Component

| Component | Gaps | Points |
|-----------|------|--------|
| A2A Worker | US-001, US-002, US-003, US-006, US-007 | 12 |
| A2A Server | US-004, US-005 | 7 |

### Recommended Sprint Planning

**Sprint 1 (Week 1-2): Foundation**
- US-001 (2 pts)
- US-003 (2 pts)
- US-002 (3 pts) - can parallel with US-003

**Sprint 2 (Week 3-4): Server & Integration**
- US-004 (3 pts)
- US-007 (2 pts)
- US-006 (3 pts)

**Sprint 3 (Week 5): Advanced Features**
- US-005 (4 pts)

---

## 6. Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Session event channel backpressure | Medium | High | Implement bounded channels with drop policies |
| SSE reconnection storms | Low | Medium | Exponential backoff, circuit breaker |
| Vault unavailability blocking registration | Low | High | Graceful degradation, continue without model info |
| Concurrent task resource exhaustion | Medium | High | Implement task limits, backpressure |
| Token usage tracking accuracy | Low | Medium | Validate against provider responses |

---

## 7. Success Criteria Summary

The A2A Ecosystem Worker Integration will be considered complete when:

1. ✅ All 7 user stories pass acceptance criteria
2. ✅ All quality checks pass (`cargo check`, `clippy`, `test`, `build`)
3. ✅ Worker can execute tasks with real-time streaming
4. ✅ Server can handle message/send and message/stream requests
5. ✅ Heartbeat keeps worker registered
6. ✅ Models and capabilities are reported on registration
7. ✅ No panics, graceful error handling throughout

---

## 8. Next Steps

1. **Immediate:** Begin with US-001 (Wire Session Execution) - unblocks all other work
2. **Parallel Track:** US-003 (Heartbeat) can proceed independently
3. **Review:** Conduct architecture review before US-005 (streaming endpoint)
4. **Testing:** Implement integration tests as each gap closes

---

*Document generated from docs/gaps-closing.json analysis*
