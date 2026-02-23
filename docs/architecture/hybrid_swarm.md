# CodeTether Hybrid Swarm Architecture

This document outlines the architecture for a zero-latency, cost-minimized, hybrid AI coding swarm that completely separates strategic reasoning from syntax formatting and code generation.

## Part 1: The OKR (Objectives and Key Results)

**Objective**: Deploy a zero-latency, cost-minimized, hybrid AI coding swarm that completely separates strategic reasoning from syntax formatting and code generation.

- **Key Result 1 (Cost)**: Reduce Claude Opus API output token costs by 100% for tool-calling and JSON formatting by offloading all syntax to local FunctionGemma.
- **Key Result 2 (Latency)**: Achieve sustained code-generation speeds of >30 Tokens Per Second (TPS) by isolating the Qwen 7B worker model entirely within the RTX 2070's 8GB VRAM.
- **Key Result 3 (Reliability)**: Achieve a 0% A2A protocol parsing failure rate by using FunctionGemma as an absolute two-way semantic filter between raw text and strict JSON.

## Part 2: Product Requirements Document (PRD)

**Project Name**: Project CodeTether Swarm (Hybrid A2A Setup)
**Status**: Architecture Finalized

### 1. Hardware & Deployment Allocation

- **Node A (The Command Center - 8GB Mac)**: Runs the CodeTether Orchestrator, the Terminal User Interface (TUI), and hosts FunctionGemma (270M) via Candle for instant, local semantic translation (5–50ms latency).
- **Node B (The Execution Engine - PC w/ RTX 2070 8GB)**: Acts as a dedicated local API server running an aggressively quantized Qwen 2.5 Coder (7B Q4_K_M) to fit entirely in VRAM.
- **Node C (The Cloud - Anthropic API)**: Hosts Claude 3 Opus, operating strictly as the strategic planner and debugger.

### 2. Data Flow & Translation Pipeline

- **Step 1 (Intent)**: Opus outputs raw, conversational English instructions.
- **Step 2 (Downward Translation)**: FunctionGemma (Mac) intercepts Opus's English, mapping it to strict Agent-to-Agent (A2A) JSON schema.
- **Step 3 (Delegation)**: CodeTether routes the JSON task over the local network to the PC.
- **Step 4 (Generation)**: Qwen (PC) generates raw Python/Rust code (plus potential conversational fluff) and returns it.
- **Step 5 (Upward Translation)**: FunctionGemma (Mac) intercepts Qwen's response, strips conversational fluff, and packages the pure code into a clean JSON payload.
- **Step 6 (Execution)**: CodeTether safely executes the clean code in the Mac's terminal.

## Part 3: The Claude Opus System Prompt

This prompt is designed to absolutely forbid Opus from trying to be helpful with formatting, forcing it to act purely as a high-level manager to save API tokens.

### System Prompt: CodeTether Swarm Orchestrator

You are the Strategic Orchestrator for a local AI software development swarm.

**YOUR ROLE:**
Your only job is to reason through software requirements, architect solutions, and issue step-by-step commands to your sub-agents. You are the "Brain." You do not have hands.

**STRICT PROTOCOLS (READ CAREFULLY):**

- **DO NOT WRITE JSON:** You are communicating through a local FunctionGemma translation layer. You must never attempt to format your outputs as JSON, XML, or structured tool calls. Do not write curly braces or schemas. Doing so will break the translation pipeline and waste tokens.
- **DO NOT WRITE CODE:** You must not write the actual implementation code. You must delegate code generation to the local worker agent.
- **USE PLAIN ENGLISH:** Issue your directives in concise, direct natural language. Treat the system like a human developer sitting next to you.

**EXAMPLE GOOD OUTPUT:**
"Agent, we need to parse the auth.log file. Write a Python script to extract all failed login IP addresses, save it to a new file called 'banned_ips.txt', and execute it."

**EXAMPLE BAD OUTPUT:**
```json
{
  "tool": "write_script",
  "code": "..."
}
```
(Do NOT do this).

State your intent clearly, wait for the worker to execute, and evaluate the terminal output when it is returned to you.
