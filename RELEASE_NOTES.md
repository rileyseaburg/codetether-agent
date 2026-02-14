# v3.0.1

## What's New

- **S3 Bus Sink**: Export training data directly to S3 with configurable bucket paths and automatic batching
- **OKR Tool**: Track objectives and key results with support for runs, checkpoints, and execution stats
- **Ralph `/go` Command**: Execute PRD-driven development workflows directly from the CLI with bus-based learning
- **Telemetry Module**: Comprehensive telemetry collection with configurable export targets
- **Provider Metrics**: Detailed usage and cost tracking across all LLM providers
- **Tool Registry API**: Programmatic access to discover and invoke registered tools
- **Relay Teams**: Team-based coordination and relay messaging support
- **TUI Bus Log**: Real-time visualization of bus messages and events in the terminal UI

## Changes

- Updated Claude Opus 4.6 Bedrock pricing to $5/$25 per million tokens
- Increased parallelism with new `ToolOutputFull` bus message type
- Enhanced Ralph loop with extended PRD execution capabilities
- Improved MCP server transport handling and type definitions
- Version bump to 3.0.1
