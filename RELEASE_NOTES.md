# v3.1.1

## What's New

- **Swarm System Expansion**: Major enhancements to the swarm orchestration system with new `collapse_controller.rs` (590 lines), `kubernetes_executor.rs` (133 lines), and `remote_subtask.rs` (209 lines) modules for advanced parallel sub-agent coordination
- **Kubernetes Integration**: Significant expansion of `src/k8s/mod.rs` (+293 lines) with enhanced cluster management and deployment capabilities
- **Voice Planning**: Added `ll-voice-plan.md` documenting voice feature roadmap

## Changes

- **Removed OpenCode Module**: Deleted `src/opencode/mod.rs` (509 lines) as part of codebase consolidation
- **Refactored Copilot Provider**: Reorganized `src/provider/copilot.rs` for improved maintainability
- **Session Management**: Streamlined session module with reduced complexity
- **Documentation Updates**: Refreshed README, IMAGE_INPUT_REQUIREMENTS, and benchmark results
- **PRD Management**: Added new PRD files and removed obsolete `prd_opencode_parity_missing_features.json`
- **Tool Improvements**: Updates to `go.rs`, `advanced_edit.rs`, and CLI run command

## Stats

```
41 files changed, 3,286 insertions(+), 1,731 deletions(-)
```
