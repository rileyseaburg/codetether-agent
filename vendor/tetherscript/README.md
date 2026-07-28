# tetherscript

tetherscript is a small, embeddable scripting language for Rust hosts and AI-agent
workflows. It is dynamically typed, uses Rust-like syntax, and is implemented in
Rust with two runtimes: a tree-walking interpreter and a stack-based bytecode VM.

The package, library, and binary are named `tetherscript`.

tetherscript is meant for project policy, validators, workflow glue, plugin hooks,
and other fast-changing behavior that should not require rebuilding a Rust
application.

## Status

tetherscript `0.1.0-alpha.16` is the current release candidate for crates.io.

tetherscript currently includes:

- A tree-walking interpreter used as the reference runtime.
- A stack-based bytecode VM targeting the same observable semantics, with local
  slots for function parameters and locals plus constant-pool deduplication.
- Dynamic values: integers, floats, booleans, strings, lists, maps, functions,
  native functions, `nil`, and `Result`.
- Variables, lexical scopes, functions, closures, recursion, `if`/`else`,
  `while`, and `for x in iterable` loops.
- Runtime ownership tracking with explicit `move` and use-after-move errors.
- `Result` values and `?` propagation for expected failures.
- Method calls on built-in values.
- JSON parsing/encoding implemented in-tree.
- Blocking HTTP client helpers, a blocking HTTP/1.1 handler server, and a
  native cached static-file server.
- Experimental browser primitives for parsing small HTML fragments, applying
  simple CSS rules, computing block layout, and rendering deterministic text
  display lists.
- A dependency-free JavaScript host for those browser primitives, with `document`,
  `window`, selectors, attributes, inline `<script>` execution, text/attribute
  mutation, basic DOM tree mutation APIs, synchronous DOM events,
  `location`/`navigator` globals, `this`, `typeof`, function expressions,
  classic `for` loops for NodeList-style iteration, and deterministic
  timers, microtasks, Promise reactions, `await`, `fetch`, and
  `XMLHttpRequest` lifecycle draining after scripts.
- Browser JavaScript compatibility for common production bundle constructs used
  by Vite/React login flows, including arrow functions, classes, optional
  chaining, nullish coalescing, dynamic import, spread/rest, template literals,
  regex route matching, Promise adoption, and XHR response fields.
- Native browser-agent APIs for agent validation workflows, including page
  loading from provided HTML, external resource registration, resource
  validation, stylesheet/script inlining, classic and module script execution,
  static module import rewriting, default ESM import/export rewriting, dynamic
  `import()` resolution, DOM assertions, screenshots/traces, production debug
  reports, HAR-style network exports, source-mapped runtime error locations and
  generated stack frames, React root/render/hydration diagnostics, classified
  runtime exceptions, computed-style plus layout evidence for rendered elements,
  React-style controlled form interaction coverage, fetch/XHR server-cookie
  propagation for auth flows, and explicit tests that reject external browser
  engines and remote-control drivers as required browser backends.
- SMTP sending support.
- Standard tools for filesystem, process, environment, path, time, Base64,
  SHA-256, and URL parsing.
- A Rust plugin host for loading tetherscript source and calling named hooks.
- A small LSP server plus VSCode extension.

The long-term direction is controlled extensibility for Rust applications:
tetherscript source is editable and inspectable, while the Rust host remains
responsible for trust, capabilities, auditing, and resource budgets.

## Quick start

```bash
cargo install tetherscript --version 0.1.0-alpha.16

cargo build --release

# Run with the bytecode VM (default)
./target/release/tetherscript run examples/hello.tether
./target/release/tetherscript run examples/fib.tether

# Run with the reference interpreter
./target/release/tetherscript run --interp examples/hello.tether
./target/release/tetherscript run --interp examples/fib.tether

# Inspect the frontend / compiler output
./target/release/tetherscript inspect --tokens   examples/hello.tether
./target/release/tetherscript inspect --ast      examples/hello.tether
./target/release/tetherscript inspect --bytecode examples/hello.tether

# Serve LSP over stdio for editors
./target/release/tetherscript lsp
```

## Language example

```kl
fn fib(n) {
    if n < 2 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println(fib(10))
}
```

## HTTP server

tetherscript ships with a dependency-free blocking HTTP/1.1 server exposed as
`http_serve`:

```kl
fn handle(req) {
    let resp = map()
    resp.status = 200
    resp.body = "hello from tetherscript\n"
    return resp
}

fn main() {
    http_serve(8787, handle)
}
```

The handler receives a request map with `method`, `path`, `query`, `headers`, and
`body`. It may return either a string, sent as `200 text/plain`, or a map with
optional `status`, `headers`, and `body`.

For static sites, `http_serve_static(port, root_dir)` preloads files through the
granted `fs` capability and serves precomputed responses from native Rust without
calling a tetherscript handler per request:

```kl
fn main() {
    http_serve_static(8788, "dist")
}
```

Run it with a filesystem grant scoped to the site root:

```bash
./target/release/tetherscript run --grant-fs examples/content_site \
  examples/static_site_server_native.tether
```

## Standard tools

The runtime includes dependency-free standard tools exposed as built-ins,
including:

- `json_parse`, `json_encode`, `json_encode_pretty`
- `http_get`, `http_head`, `http_post`, `http_request`, `http_serve`,
  `http_serve_static`
- `browser_parse_html`, `browser_parse_css`, `browser_styles`,
  `browser_query_selector`, `browser_text_content`, `browser_snapshot`,
  `browser_layout`, `browser_display_list`, `browser_render`
- `js_eval`, `browser_eval_js`, `browser_run_scripts`,
  `browser_compatibility_report`
- `smtp_send`
- `fs_read`, `fs_write`, `fs_exists`, `fs_list`, `fs_mkdir`, `fs_stat`,
  `fs_remove`, `fs_rename`, `fs_copy`
- `process_run`, `process_args`, `process_pid`, `process_platform`,
  `process_arch`
- `env_get`, `cwd`, `chdir`
- `path_join`, `path_dirname`, `path_basename`, `path_extname`,
  `path_normalize`, `path_resolve`, `path_sep`
- `time_now_ms`, `sleep_ms`
- `sha256_hex`, `base64_encode`, `base64_decode`, `url_parse`

See [`docs/standard-tools.md`](docs/standard-tools.md) for more detail.

## Plugin embedding

tetherscript can be embedded as a local plugin language. A Rust host can load
tetherscript source, grant host-provided authority, and call named functions as
hooks.

This is the intended boundary:

```text
Rust host / agent system -> tetherscript hook -> granted host capability -> structured result
```

See [`docs/tetherscript-and-codetether.md`](docs/tetherscript-and-codetether.md)
for the CodeTether integration model.

## Editor support

A VSCode extension lives in [`editor/vscode/`](editor/vscode/). It provides syntax
highlighting and connects VSCode to `tetherscript lsp` for live lex/parse
diagnostics.

## Design principles

- **Dynamic types** — no type annotations are required.
- **Rust-like syntax** — `fn`, `let`, `let mut`, braces, `&`, `&mut`, `move`,
  and expression-oriented blocks.
- **Runtime-checked ownership** — heap values can be moved; use-after-move is
  reported at runtime.
- **Copy scalars, move heap values** — scalar values remain usable after `move`;
  lists/maps/strings transfer ownership.
- **Recoverable errors** — `Result` and `?` handle expected failure; `panic` is
  for bugs.
- **Embeddable host boundary** — host effects should flow through explicit host
  integration rather than unreviewable shell glue.
- **Zero-dependency goal** — the runtime avoids third-party crates unless a
  future governance decision justifies them.

## What is not done yet

- Runtime `&mut` aliasing / XOR-mutability enforcement.
- Modules and imports.
- Formatter and REPL.
- More complete LSP features such as completions, hover, go-to-definition, and
  exact spans.
- Complete Test262 and Web Platform Tests coverage. The JavaScript/browser
  runtime is an in-tree, zero-dependency compatibility track and currently
  implements a practical DOM/JS subset rather than every web standard.
- Full browser networking, navigation, DOM, CSS, rendering, JS, and Web API
  parity. The browser track is intended to become a native full-parity browser
  for agents, with conformance tracked explicitly rather than delegated to an
  external browser engine.
- Capability audit logs and richer resource budgets.
- Moving ambient host tools behind explicit capabilities where practical.
- Async scheduler.

## Repository layout

```text
src/
  ast.rs         — AST node definitions
  browser.rs     — experimental HTML/CSS parser, layout, and text renderer
  browser_js.rs  — dependency-free browser JavaScript DOM host bindings
  bytecode.rs    — bytecode instruction/chunk/function types
  capability.rs  — capability trait/object model
  compiler.rs    — AST to bytecode compiler
  http.rs        — HTTP built-in module surface
  http_client.rs — plain HTTP client helpers
  http_server.rs — dynamic handler HTTP server
  http_static/   — native cached static-file HTTP server
  interp.rs      — tree-walking interpreter
  json.rs        — in-tree JSON parser/encoder
  lexer.rs       — lexer
  lib.rs         — library surface for embedding
  lsp.rs         — LSP server
  output.rs      — output capture helpers
  parser.rs      — parser
  plugin.rs      — Rust host plugin API
  smtp.rs        — SMTP support
  system.rs      — filesystem/process/env/path/time/hash/base64/url tools
  token.rs       — token types
  value.rs       — runtime values, ownership slots, environments
  vm.rs          — bytecode VM
examples/
  hello.tether, fib.tether, closures.tether, ownership.tether, use_after_move.tether
editor/vscode/
  VSCode grammar and LSP client
docs/
  standard tools and CodeTether integration notes
```

## Roadmap

- [x] Lexer
- [x] Parser and AST
- [x] Tree-walking interpreter
- [x] Runtime ownership tracking
- [x] Bytecode compiler and VM
- [x] LSP server and VSCode extension
- [x] `Result` and `?` semantics
- [x] `for x in iterable` loops
- [x] JSON support
- [x] HTTP client/server support
- [x] Standard filesystem/process/env/path/time/hash/base64/url tools
- [x] Experimental browser parser/layout/text renderer
- [x] Experimental dependency-free browser JavaScript host bindings
- [x] Native production debug report for bundled UI validation
- [x] Rust embedding/plugin host
- [ ] Native full browser parity
- [ ] Runtime `&mut` exclusivity enforcement
- [ ] Modules and imports
- [ ] Plugin and capability manifests as stable formats
- [ ] Audit stream for capability calls
- [ ] Formatter
- [ ] REPL
- [ ] Async scheduler
