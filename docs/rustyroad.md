# First-party RustyRoad access

CodeTether registers a built-in `rustyroad` tool in its standard tool registries.
It calls the `rustyroad-mcp` executable published with
[RustyRoad on crates.io](https://crates.io/crates/rustyroad). There is no
checkout-path dependency in CodeTether's Cargo manifest and no manual generic
MCP connection setup. RustyRoad owns database/project operations; CodeTether
owns project selection, approval enforcement, protocol handling and cleanup.

## Install

```sh
cargo install rustyroad --locked --bin rustyroad-mcp
rustyroad-mcp --help
```

Ensure `rustyroad-mcp` is on the PATH used to launch CodeTether. The tool reports
the installation command if the executable is missing. For development against
the sibling checkout, use `cargo install --path ../../RustyRoad --locked --bin
rustyroad-mcp` instead. CodeTether never installs or upgrades the backend during
a tool call.

## Discover capabilities

Supply the RustyRoad project directory explicitly. The default environment is
`dev`, regardless of the agent process's ambient `ENV` or `ENVIRONMENT`.

```json
{
  "action": "list_tools",
  "cwd": "/path/to/rustyroad-project",
  "justification": "Discover RustyRoad operations for this project"
}
```

Discovery returns the installed version's tool names, descriptions and input
schemas. Use these schemas rather than assuming every RustyRoad release has
the same capabilities. Calls are restricted to advertised `rustyroad_*` tools.

## Call an operation

Example `rustyroad` arguments (not generic `mcp` arguments):

```json
{
  "action": "call_tool",
  "cwd": "/path/to/rustyroad-project",
  "environment": "test",
  "tool_name": "rustyroad_config",
  "arguments": {},
  "justification": "Inspect the test database configuration without connecting"
}
```

Select `dev`, `test` or `prod` explicitly. The adapter forwards this selection
as the remote tool's `env` argument. A conflicting nested `arguments.env` is
rejected rather than silently switching databases. Configure credentials using
RustyRoad's normal project configuration; never put secrets in prompts.
Selecting `dev` does not guarantee safety if that project's dev configuration
itself points to a production database.

The model can discover schemas and then use queries, schema inspection,
migrations, scaffolding or project operations supported by its installed
RustyRoad version. For example:

```sh
codetether run "Use rustyroad in /path/to/project, environment test. Discover its tools, then inspect the schema. Do not migrate or write data."
```

## Approval and isolation

- Every RustyRoad invocation is conservatively classified as mutating,
  including discovery: it obeys configured access mode, permissions and trust.
- Ask mode requires `justification`; retry an approved invocation with the
  returned `approval_id`. Approval covers project, environment, operation and
  arguments, so changing any of these requires a new approval.
- Approval is checked before the backend starts, including direct tool calls.
- Each invocation launches a fresh fixed executable, never a shell command.
  Its cwd and environment are child-only; CodeTether's process state is unchanged.
- MCP errors remain unsuccessful tool results. Output lines are limited to
  1 MiB, and an invocation times out after 120 seconds.
- CodeTether kills/reaps the child after an invocation and owns its process tree
  for cancellation cleanup. This is lifecycle isolation, not an OS sandbox.
- Results carry `rustyroad_version`, `rustyroad_project` and
  `rustyroad_environment` metadata for attribution.

## Focused regression and local smoke tests

```sh
cargo test --lib rustyroad -- --test-threads=1
cargo test --lib rustyroad_installed_discovery_and_config_smoke -- --ignored --test-threads=1
```

The default tests include mocked local MCP protocol coverage. The opt-in smoke
test uses the actual installed executable and temporary, nonsecret dev/test
configurations, with no live database connection or migration. Neither test
constitutes live deployment or database proof.