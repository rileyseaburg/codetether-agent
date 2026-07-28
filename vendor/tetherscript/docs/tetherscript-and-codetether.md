# TetherScript Inside CodeTether

TetherScript fits CodeTether as a small, source-level extension runtime.

CodeTether is the agent system. It owns conversations, model calls, tool
registration, approval policy, workspace state, audit trails, and the durable
parts of orchestration. TetherScript is not another agent framework inside it. TetherScript is
the language CodeTether can run when a repository, maintainer, or agent needs
to define behavior that is too project-specific or fast-changing to compile
into CodeTether itself.

That distinction matters. CodeTether should remain the trusted Rust host. TetherScript
should become the controlled scripting layer at the edge of that host.

The simple version:

```text
model reasoning -> CodeTether orchestration -> TetherScript hook -> granted capability
```

The model decides what it wants to do. CodeTether decides whether that shape of
work is allowed. TetherScript expresses the local project behavior. Capabilities perform
the actual effects.

## The Job TetherScript Does

Rust is the right language for CodeTether's core. It is strong at building a
reliable engine: provider clients, session state, audit storage, tool schemas,
worktree management, browser control, policy enforcement, server endpoints, and
long-running process supervision.

Rust is not the right place for every repository-specific rule. A team should
not need to patch and rebuild CodeTether to say:

- This repository requires a specific validation sequence.
- This kind of generated file must be rejected.
- This tool result needs to be transformed before the model sees it.
- This project has a custom definition of "safe command".
- This package manager has a special lockfile rule.
- This agent should ask for human approval before touching these paths.
- This PR is acceptable only if these local checks pass.

Those are not CodeTether engine concerns. They are local policy and workflow
concerns. TetherScript gives CodeTether a place to run them.

The point is not to make CodeTether less Rust-native. The point is to keep the
Rust-native core stable while allowing local behavior to evolve.

## The Boundary

CodeTether should call TetherScript through explicit hooks. A hook is a named function
with a known input shape and a known output shape.

Examples:

```text
validate(change) -> result
classify_task(request) -> result
guard_tool_call(call) -> result
summarize_evidence(files) -> result
score_patch(patch) -> result
route_agent_task(task) -> result
transform_tool_output(output) -> result
```

TetherScript should not reach randomly into CodeTether internals. CodeTether should not
pretend TetherScript is a trusted Rust module. The host passes values in, grants narrow
capabilities, runs the hook, captures the result, and records what happened.

That boundary is the product.

If the boundary is clean, CodeTether gains a plugin system. If the boundary is
blurry, CodeTether gains another source of unreviewable behavior.

## Capabilities Are The Contract

TetherScript should not be useful because it has ambient access. It should be useful
because CodeTether can grant exactly the power a hook needs.

A repository validation hook might receive:

```text
project.read
project.diff
project.search
test.run
```

A patch-generation guard might receive:

```text
policy.path_rules
audit.emit
approval.request
```

A browser workflow hook might receive:

```text
browser.snapshot
browser.click
browser.type
browser.network_log
```

A tool adapter hook might receive:

```text
json.parse
json.encode
tool.call
audit.emit
```

The hook can only do what the host granted. That is how TetherScript should fit
CodeTether's safety model.

The future shape is not "TetherScript has fs_read and process_run everywhere." The
future shape is "CodeTether grants a project-scoped filesystem capability or a
specific command capability when that is the right thing to do."

## What A Repository Gets

A repository should be able to carry CodeTether behavior the same way it carries
tests, config, lint rules, and CI workflows.

One possible layout:

```text
.codetether/
  tetherscript/
    plugin.toml
    validate.tether
    guard.tether
    summarize.tether
```

The manifest describes the hooks:

```toml
name = "repo-policy"
version = "0.1.0"

[[hook]]
name = "validate"
source = "validate.tether"
inputs = ["change"]
requires = ["project.read", "project.search", "test.run"]

[[hook]]
name = "guard_tool_call"
source = "guard.tether"
inputs = ["call"]
requires = ["policy.path_rules", "approval.request"]
```

This gives maintainers something concrete to review. They can see which hooks
exist, what capabilities they ask for, and what code will run.

That is better than burying project-specific behavior in prompts. It is better
than asking the model to remember policy. It is better than giving an agent a
shell and hoping the prompt is strong enough.

## Where MCP Fits

MCP is useful when CodeTether wants to expose tools to external AI clients or
consume tools from external systems.

TetherScript is useful behind that tool boundary.

For example, CodeTether can publish an MCP tool named `repo_validate`. The
external client sees a normal MCP tool with a JSON schema. Internally,
CodeTether can implement the behavior by calling a TetherScript `validate` hook with
project capabilities.

```text
MCP client -> CodeTether MCP server -> TetherScript validate hook -> project capabilities
```

MCP standardizes the outer protocol. TetherScript customizes the inner behavior.

That keeps the public interface stable while letting each repository define its
own validation logic.

## Where A2A Fits

A2A is useful when CodeTether needs to communicate with peer agents as agents,
not just call them as stateless tools.

TetherScript can help define local skills exposed through that agent interface.

For example, CodeTether might expose an A2A skill called "review this Rust
change." The A2A layer handles task exchange, identity, messages, and artifacts.
The local implementation can use TetherScript to decide what repository checks to run,
how to summarize evidence, and when to ask for approval.

```text
remote agent -> A2A task -> CodeTether -> TetherScript skill policy -> CodeTether tools
```

A2A coordinates agents. TetherScript defines local project behavior.

## Where OpenAI Tools Fit

When CodeTether is used inside an OpenAI Agents SDK flow, the same split holds.

The SDK owns the agent loop. CodeTether owns the local tool implementation.
TetherScript can sit behind a CodeTether tool when behavior should be editable by a
repository or generated by an agent.

```text
OpenAI Agent -> function tool -> CodeTether tool -> TetherScript hook -> capability
```

TetherScript does not need to know about every model provider. It needs a stable value
boundary so CodeTether can call it from any provider path.

## Why Not Just Use JavaScript Or Python

JavaScript and Python are strong general scripting ecosystems. That is also the
problem for this use case.

CodeTether does not need an embedded runtime with a large package ecosystem, a
large standard library, ambient filesystem access, subprocess culture, and a
large dependency footprint. CodeTether needs a small language that is easy for
agents to write and easy for hosts to constrain.

TetherScript should be intentionally less magical than Python and Node. It should make
the host boundary visible. It should make capabilities explicit. It should make
generated source small enough to audit.

If a task requires the full Node ecosystem, CodeTether can call Node as a tool
with approval. TetherScript is for the behavior that should live inside CodeTether's
trust boundary.

## Why Not Just Use Wasm

Wasm is a strong answer for compiled, portable plugins. It is less ideal when
the thing being produced is a small project-specific policy script written by an
agent during a session.

Wasm wants a build step. TetherScript wants an edit-run loop.

Wasm is a binary artifact. TetherScript is source that a maintainer can read.

Wasm is good for reusable plugin packages. TetherScript is good for local policy,
workflow glue, validators, adapters, and generated hooks.

CodeTether can eventually support both. They solve different problems.

## What Should Move Into TetherScript First

The first CodeTether-TetherScript use cases should be narrow and testable.

Good first hooks:

- `validate_patch`: receives changed files and returns pass/fail with evidence.
- `guard_tool_call`: receives a proposed tool call and returns allow, deny, or require approval.
- `summarize_files`: receives selected file contents and returns a structured summary.
- `classify_task`: receives a user request and returns a task class or routing hint.
- `score_risk`: receives a planned action and returns a risk level plus reasons.

Bad first hooks:

- Long-running autonomous agents.
- Unbounded shell wrappers.
- Network crawlers.
- General package managers.
- Anything that needs broad ambient authority.

TetherScript should earn trust by doing small, useful, reviewable things first.

## What CodeTether Must Provide

For TetherScript to matter inside CodeTether, CodeTether needs a host-side contract.

It should provide:

- A plugin loader for repository-local TetherScript files.
- A hook registry that maps CodeTether events to TetherScript functions.
- A manifest format for hook names, input schemas, output schemas, and required capabilities.
- A value bridge between JSON and TetherScript values.
- A capability bridge from CodeTether tools into TetherScript authority values.
- A budget system for steps, wall time, output size, and host calls.
- An audit stream for every capability call.
- Error formatting that points an agent back to the TetherScript source location.
- Tests that run the same hook through the interpreter and VM when possible.

This is the part that makes TetherScript first-class in CodeTether rather than a side
experiment.

## What TetherScript Must Provide

TetherScript needs to become more predictable as a host-embedded language.

It should provide:

- Stable embedding APIs.
- Stable plugin call semantics.
- Full `Result` and `?` behavior.
- Better stack traces and source spans.
- Runtime mutable-borrow enforcement.
- Modules and imports.
- A formatter.
- A REPL for local development.
- A test runner for `.tether` files and legacy `.tether` files.
- A capability manifest model.
- Interpreter and VM semantic parity.

The language does not need every feature from Node or Python. It needs the
features that make repository-local agent behavior safe and pleasant.

## The Product Shape

The product shape is not "install TetherScript and write scripts."

The product shape is:

```text
CodeTether loads repository behavior.
Repository behavior is written in TetherScript.
TetherScript behavior declares required capabilities.
CodeTether grants or denies those capabilities.
Agents can generate or update the TetherScript behavior.
Maintainers can read and review the TetherScript behavior.
CodeTether can audit every host effect.
```

That is the value loop.

It gives agents a way to become more useful inside a repository without being
given the keys to the entire machine.

It gives maintainers a way to encode project knowledge in executable form
without waiting for CodeTether core changes.

It gives CodeTether a plugin story that stays aligned with Rust's safety and
supply-chain values.

## The Main Risk

The main risk is turning TetherScript into an undisciplined mini-Node.

If TetherScript accumulates broad ambient APIs, a package ecosystem before a security
model, and unclear host boundaries, it loses the reason it exists.

The second risk is making TetherScript too abstract. If it becomes only a philosophy of
capabilities and never solves boring developer needs, teams will not use it.

The balance is practical:

- Give scripts enough standard tools to be useful.
- Move real authority behind host-granted capabilities.
- Keep the runtime dependency-free.
- Keep the source readable.
- Keep the embedding API boring.
- Keep the CodeTether integration concrete.

## The Near-Term Path

The next useful CodeTether milestone is not more theory. It is a complete
repository-local TetherScript plugin flow.

The flow should look like this:

```text
codetether discovers .codetether/tetherscript/plugin.toml
codetether validates requested capabilities
codetether loads the declared TetherScript hooks
codetether exposes one hook as tetherscript_plugin or an internal policy hook
codetether runs the hook with JSON input
tetherscript returns a structured result
codetether records output, errors, and capability calls
tests prove the flow works
```

Once that exists, the ecosystem story becomes real. TetherScript is not just a language
inside a repo. It is the way CodeTether lets repositories participate in the
agent loop.

## The Long-Term Direction

Long term, TetherScript can become the local behavior layer for agentic Rust systems.

CodeTether is the first serious host because it already has the right problem:
agents need tools, tools need policy, policy needs to be editable, and Rust
core changes are too heavy for every local rule.

If TetherScript works there, the same pattern applies elsewhere:

- Devtools that need repository-local automation.
- CI systems that need safe, project-defined checks.
- IDEs that need user-extensible code actions.
- Agent platforms that need local policy and adapters.
- Rust services that need customer-specific workflow hooks.

The meaning of TetherScript is not that the world needs another scripting language.

The meaning is that agent systems need a safer way to execute generated local
behavior, and Rust hosts need an extension layer that does not betray Rust's
values.

Inside CodeTether, TetherScript is that layer.
