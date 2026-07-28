# Agent Browser Contract

The tetherscript browser track is the preferred browser surface for agents when
the job needs deterministic observation, explicit capabilities, replayable
actions, and testable output.

It has two stable entry points:

- `tetherscript render` and `tetherscript raster` for deterministic offline
  HTML/CSS rendering.
- `tetherscript::browser_agent::BrowserPage` for embedded observe, act, assert,
  screenshot, event-log, trace, and deterministic resource-validation workflows.

Browser control remains capability-gated through `BrowserAuthority` and
tetherscript browser action endpoints. Scripts must be granted origins and
scopes before navigation, interaction, network inspection, storage mutation,
visual lookup, or screenshot actions can cross that boundary.

Promotion rule: every agent-browser behavior that is recommended in docs needs
a local contract test. CLI behavior belongs in `tests/agent_browser_cli.rs`.
Embedded page behavior belongs in `tests/agent_browser_page.rs`. Action-envelope
behavior belongs in `tests/browser_cap_contract.rs`, and endpoint smoke tests
remain opt-in through `TETHERSCRIPT_BROWSERCTL_ENDPOINT`.

The browser track must not depend on external browser engines or automation
adapters. Production UI validation is done by closing gaps in tetherscript's
native DOM, CSS, JavaScript, networking, storage, accessibility, screenshot, and
trace implementation.

`BrowserPage::production_debug_report()` is the preferred one-shot native
debug artifact for production React-style pages. It now includes visual element
evidence that joins selector candidates, computed styles, visibility, and
layout bounds so an agent can distinguish "React mounted" from "React mounted
but the UI is hidden, zero-sized, or styled incorrectly."

Agent actions must behave like user input for production form workflows:
controlled inputs read live DOM values from captured element handles, click
actions dispatch pointer/mouse/focus/click ordering, submit prevention blocks
native navigation, and Enter on an input submits the containing form through the
native runtime.

Auth-bearing network flows must preserve browser cookie semantics across the
native route layer. `fetch` and XHR responses that include `Set-Cookie` update
the page session jar, `HttpOnly` cookies stay hidden from `document.cookie`, and
later same-session requests receive a generated `Cookie` header.

## Readiness Suite

Run the deterministic and bridge-contract browser tests with:

```bash
cargo test --test agent_browser_cli --test agent_browser_page \
  --test browser_cap_contract --test browser_assertions \
  --test browser_cli_grant --test browser_trace_contract \
  --test agent_browser_resources --test agent_browser_modules \
  --test agent_browser_bundle_exports --test agent_browser_dynamic_import \
  --test agent_browser_native_contract \
  --test agent_browser_auth_cookies \
  --test browser_js_promise_await --test browser_js_xhr_parity
```

Run the endpoint smoke test only when a tetherscript browser endpoint is
available:

```bash
TETHERSCRIPT_BROWSERCTL_ENDPOINT=http://127.0.0.1:41707/browser \
  cargo test --test browser_cap_live
```

The live host must accept action envelopes for the documented methods and
return either raw JSON, `{ "ok": true, "value": ... }`, or CodeTether-style
`{ "success": true, "output": ... }` tool results.
