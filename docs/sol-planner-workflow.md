# Tool-Free Sol Planner Workflow

Use `--sol-planner` with a separate worker model:

```bash
codetether run --sol-planner \
  --model openrouter/your-fast-or-free-coding-model \
  "Implement the requested change"
```

The workflow pins planning and LSP-diagnostic review to
`openai-codex/gpt-5.6-sol-fast:max`. Sol receives an empty tool list and its
responses are never interpreted as executable tool calls. The selected
`--model` is the only model that receives workspace tools and can write code.

After the first coding pass, CodeTether runs its LSP/linter validation over the
files changed by structured worker tools. Any unresolved diagnostics are passed
to Sol as data for a repair plan, then returned to the coding worker. The loop
performs at most two Sol-guided repair rounds and reports any remaining count.

`--sol-planner` requires an explicit `--model`; using Sol itself as the worker
is rejected so the planning and coding roles cannot collapse accidentally.
