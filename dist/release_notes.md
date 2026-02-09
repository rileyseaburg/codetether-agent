## What's Changed

### Bedrock Model Discovery Fix
- Fix: Accept INFERENCE_PROFILE inference type in model discovery (not just ON_DEMAND). All newer Anthropic models use INFERENCE_PROFILE only, confirmed via AWS CLI.
- New models now discovered: Claude Opus 4.6, 4.5, 4.1, Sonnet 4, 4.5, Haiku 4.5 (72 total Bedrock models, up from 61)

### New CLI Command
- codetether models - list all discovered models from all providers
- codetether models --provider bedrock - filter by provider
- codetether models --json - JSON output

### New Aliases
- claude-opus-4.6, claude-opus-4.5, claude-opus-4.1
- claude-sonnet-4.5, claude-haiku-4.5, claude-3.5-sonnet-v2

### Install
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh

Full Changelog: https://github.com/rileyseaburg/codetether-agent/compare/v0.1.7...v0.1.8
