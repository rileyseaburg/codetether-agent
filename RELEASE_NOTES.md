# v4.0.0

## What's New

### New Providers
- **Gemini Web Provider**: Browser-cookie authentication for Gemini models including `gemini-web-deep-think`
- **Vertex AI Anthropic Provider**: Full support for Anthropic models on Google Cloud Vertex AI with Gemini 3.x thought signatures
- **Local CUDA Provider**: Hardware-accelerated local inference with optimized TTFT (Time To First Token)
- **New Models**: Claude Sonnet 4.6, Gemini 3.1 Pro preview models

### RLM Oracle System
- **Tree-sitter Oracle**: Structural code analysis using tree-sitter parsing
- **Grep Oracle**: Pattern-based code search with validation
- **Schema Validator**: Comprehensive oracle result validation
- **Template System**: 659 lines of templates for oracle operations

### TUI Enhancements
- **Worker Bridge**: New TUI worker bridge for improved A2A worker integration
- **Token Caching**: Reduced latency with intelligent token caching
- **Interactive Bash**: Unblocked interactive bash authentication flows

### Tool Improvements
- **File Extras**: 648 new lines of extended file operations
- **FunctionGemma Router**: Heavily optimized TTFT and hardware acceleration
- **Worktree Management**: Improved Git worktree isolation for parallel development

### Telemetry & Observability
- **New Telemetry Module**: 775 lines of comprehensive telemetry support
- **Event Stream**: Improved event streaming architecture
- **Context Tracing**: 413 lines of context tracing for RLM operations

## Bug Fixes
- Fixed type inference errors across main.rs and session modules
- Addressed PR review feedback for provider implementations
- Resolved compilation errors in LocalCudaProvider integration

## Changes
- Major refactoring of RLM module structure (oracle system extracted to dedicated submodule)
- Worktree module consolidated and improved
- Session management enhancements for better state handling
- Provider registry updated with new model support
- Documentation updated for v4.0.0 with new architecture guides
- Added comprehensive test suite for oracle functionality (433 lines)
