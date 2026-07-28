# Browser WPT/Compliance Map

This inventory maps the native tetherscript browser parity track to Web
Platform Tests (WPT)-style coverage. Entries name the implemented surface, the
closest WPT area or test shape, local coverage, and known gaps that block
claiming full browser parity.

## Scope

The browser track is a native full-parity target for agents. It starts from a
deterministic in-tree implementation: HTML is parsed into a DOM, CSS is parsed
and matched, layout/rendering are computed natively, and browser APIs are
exposed to the in-tree JavaScript interpreter. Full parity means closing this
map against spec/WPT behavior without delegating execution to an external
browser engine or remote-control driver.

This document is not a pass/fail WPT report. It is a planning map for turning
local unit tests and fixtures into WPT-like compliance cases.

## Inventory

| Area | Implemented surface | WPT-like reference area | Local coverage anchor | Status | Main gaps |
| --- | --- | --- | --- | --- | --- |
| HTML tree construction | Elements, text nodes, attributes, void-ish `<br>`/`img`, basic entity decoding | `html/syntax/parsing`, `html/dom` | `src/browser.rs::parses_html_to_dom_value`, `decodes_nested_entities` | Partial | No HTML5 tokenizer/tree-construction algorithm, namespaces, comments/doctype, error recovery matrix, script/parser interaction |
| DOM querying | `querySelector`, `querySelectorAll`, `getElementById`, descendant matching | `dom/nodes`, `selectors-api` | `src/browser.rs::query_selector_*`, `src/browser_js.rs::eval_with_dom_exposes_selectors_and_attributes` | Partial | No full Selectors spec, pseudo-classes/elements, sibling/child combinators, live collections, invalid selector error taxonomy |
| Selectors/CSS cascade | Type/id/class/attribute selectors, descendant compounds, specificity, inline `style` precedence | `css/selectors`, `css/css-cascade` | `src/browser.rs::css_supports_compound_descendant_and_inline_cascade` | Partial | No origins/layers/importance/media queries/inheritance matrix/shorthand expansion/unknown-value handling |
| CSSOM/layout/rendering | Basic dimensions, background/text/image display commands, deterministic text rendering | `css/cssom`, `css/css2`, `html/rendering` | `src/browser.rs::lays_out_and_renders_text_display_list`, `display_list_contains_background_and_text_commands`, `img_src_attribute_carried_into_display_command` | Experimental | Not a CSS visual formatting model, no inline layout, positioning, flex/grid, fonts, painting order, viewport/device concepts |
| DOM text/html attributes | `textContent`, `innerText`, `innerHTML`, `children`, `getAttribute`, `setAttribute` | `dom/nodes`, `html/dom/elements` | `src/browser_js.rs::inline_scripts_can_read_document_and_console_log`, `dom_property_assignment_and_mutation_apis_update_document` | Partial | Attribute reflection is minimal, serialization order is not spec-defined, no node types beyond element/text, no mutation observers |
| DOM mutation | `createElement`, `appendChild`/append, prepend/remove helpers via host bindings | `dom/nodes/Node-appendChild`, `dom/nodes/ParentNode-*` | `src/browser_js.rs::dom_property_assignment_and_mutation_apis_update_document` | Partial | Index-path handles can shift after mutation; no clone/import/adopt/insertBefore/replaceChild, no document fragments |
| Events | `addEventListener`, `removeEventListener`, `onclick`, `dispatchEvent`, `click`, event `type`/`target`, listener `this` | `dom/events`, `uievents` smoke cases | `src/browser_js.rs::event_listeners_property_handlers_this_and_event_target_work`, `remove_event_listener_and_typeof_work` | Partial | No capture/bubble phases, propagation control, default actions, composed paths, trusted events, event subclasses |
| Window/global aliases | `window`, `self`, `document`, `navigator`, `location` | `html/webappapis`, `html/browsers/the-window-object` | `src/browser_js.rs::location_and_navigator_globals_are_available`, `window_self_and_storage_globals_are_available` | Smoke | Static localhost-only objects; no navigation/history/origin model, no frames, no real event loop |
| Timers and microtasks | Deterministic `setTimeout`, `clearTimeout`, `setInterval`, `queueMicrotask`, `requestAnimationFrame`, Promise reactions, and callback args drained after script execution | `html/webappapis/timers`, `html/webappapis/microtask-queuing` | `src/browser_js.rs::microtasks_animation_frames_and_timers_have_deterministic_order`, `tests/browser_js_promise_await.rs` | WPT-like deterministic subset | No wall-clock scheduling, clamping/nesting behavior, full task-source model, or worker timers |
| Web Storage | In-memory `localStorage`/`sessionStorage` with `getItem`, `setItem`, `removeItem`, `clear`, `key`, `length` | `webstorage` | `src/browser_js.rs::local_storage_implements_minimal_storage_api`, `session_storage_is_separate_from_local_storage_and_per_eval`, `compatibility_report_lists_storage_apis` | Partial | Per-eval ephemeral storage, no origin scoping/persistence/quota/events/security errors, no property-indexed access |
| JavaScript integration | Inline `<script>` execution, expression return value, console log capture, functions/classes, loops, modern expression syntax, `typeof`, `this` in supported callbacks, deterministic module resource expansion for default/named imports and dynamic imports, Promise adoption, `await`, `fetch`, and `XMLHttpRequest` response lifecycle fields | `html/semantics/scripting-1`, `console`, `ecmascript` host smoke, `xhr`, `fetch` | `src/browser_js.rs` unit tests, `tests/agent_browser_react_render.rs`, `tests/browser_js_promise_await.rs`, `tests/browser_js_xhr_parity.rs` | Project-specific | No full ESM loader, complete Test262 semantics, complete XHR/fetch error taxonomy, exceptions parity, async stack traces, or external WPT harness |
| Runtime builtins | `browser_parse_html`, `browser_parse_css`, `browser_styles`, `browser_query_selector`, `browser_text_content`, `browser_snapshot`, `browser_display_list`, `browser_render`, `browser_layout`, `browser_run_scripts`, `browser_eval_js`, compatibility report | Project API contract; WPT harness adapter candidates | `src/browser.rs::browser_builtins_return_values`, `browser_variadics_reject_extra_args`; `src/browser_js.rs::compatibility_report_lists_storage_apis` | Local API covered | Need stable JSON fixture format and harness glue before importing external WPT data |
| Production diagnostics | Console/page errors, HAR-style network entries, source-mapped error locations and generated stack frames, failed requests, source-map references, classified runtime exceptions, React roots and hydration warnings | `console`, `fetch`, source maps, framework integration smoke | `tests/agent_browser_production_debug.rs` | Agent-debug subset | Needs async stack frames and framework component stack reconstruction |

## Suggested WPT-like fixture layout

A future docs/tests-only step can add fixture cases without importing WPT itself:

```text
tests/browser_wpt_like/
  dom-query-basic.json          # html, script/query, expected text/counts
  events-click-basic.json       # html, script, expected value/console/dom text
  storage-basic.json            # script, expected value
  timers-deterministic.json     # script, expected console/order
  css-cascade-basic.json        # html, css, selector, expected computed fields
```

Each fixture should use a small normalized schema:

```json
{
  "area": "dom/events",
  "wpt_shape": "dispatchEvent invokes listener with target and this",
  "html": "<button id='go'>old</button>",
  "script": "let b=document.getElementById('go'); let seen=''; b.addEventListener('click', function(e){ seen=e.type + ':' + e.target.id + ':' + this.id; }); b.click(); seen;",
  "expect": { "value": "click:go:go" }
}
```

## Promotion criteria

Before claiming WPT compatibility for any row, add or link at least:

1. a WPT-like fixture with spec/WPT-area metadata;
2. a local Rust harness assertion for the fixture result;
3. documented unsupported cases for the same feature family; and
4. a stable command that runs the targeted subset in CI.
