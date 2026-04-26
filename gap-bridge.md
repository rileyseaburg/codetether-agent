# Gap Bridge: Codex Desktop Computer Use vs. CodeTether Browser Control

## Sources reviewed

- OpenAI Codex app overview: `https://developers.openai.com/codex/app`
- OpenAI Codex app features: `https://developers.openai.com/codex/app/features`
- OpenAI Codex in-app browser: `https://developers.openai.com/codex/app/browser`
- OpenAI Codex computer use: `https://developers.openai.com/codex/app/computer-use`
- OpenAI product announcement: `https://openai.com/index/codex-for-almost-everything/`
- Local implementation under `src/browser/**`, `src/tool/browserctl/**`, and `src/cli/browserctl/**`

## Executive summary

OpenAI uses two distinct automation surfaces in Codex Desktop:

1. **In-app browser / Browser plugin** for constrained web-preview workflows.
2. **Computer Use plugin** for macOS GUI automation through OS-level permissions.

Our local implementation is closer to OpenAI's **in-app browser / browser use** path than to their **Computer Use** path. CodeTether currently drives Chromium-family browsers through the Chrome DevTools Protocol (CDP), with DOM, JavaScript, network replay, screenshots, tabs, and synthetic input events. OpenAI's Computer Use is documented as an OS-mediated desktop automation layer that can see and operate arbitrary macOS apps after Screen Recording and Accessibility permissions are granted.

The bridge is to keep `browserctl` as the structured browser automation layer, then add a separate `computer_use` capability with OS adapters, permission checks, app allowlists, screen capture, native accessibility actions, and stricter approvals.

## What OpenAI is doing

### In-app browser

OpenAI's in-app browser is a shared browser view inside a Codex thread. It is intended for local development servers, file-backed previews, and public unauthenticated pages. Their docs state it does not support regular browser profiles, existing tabs, cookies, extensions, or signed-in pages. Browser use operates inside that constrained in-app browser and is controlled through a Browser plugin with allowed and blocked websites.

Key traits:

- Thread-scoped browser surface.
- Best for localhost and unauthenticated pages.
- Supports browser comments for precise visual feedback.
- Browser use can click, type, inspect rendered state, and take screenshots.
- Site-level allow/block controls.
- Explicit warning to treat page content as untrusted context.

### Computer Use

OpenAI's Computer Use plugin is documented as macOS-only at launch, with regional limitations. It requires macOS Screen Recording permission for perception and Accessibility permission for control. It can see and operate graphical user interfaces, including desktop apps, browsers, simulators, app settings, and workflows spanning more than one app.

Key traits:

- Uses OS-level permission gates: Screen Recording and Accessibility.
- Can view screen content and interact with windows, menus, keyboard input, pointer input, and clipboard state.
- Targets arbitrary allowed apps rather than only a CDP-attached Chromium tab.
- Has app-level approvals and an “Always allow” list.
- Can ask before sensitive or disruptive actions.
- Cannot automate terminal apps or Codex itself, and cannot approve administrator/security/privacy prompts.
- Intended for scoped GUI tasks where CLI tools and structured integrations are insufficient.
- Product announcement claims background computer use with its own cursor and parallel agents on Mac without interfering with the user's work in other apps.

## What CodeTether does locally today

### Browser control surface

CodeTether exposes `browserctl` as both a tool and CLI command. The local code says it controls a Chromium browser through the local DevTools Protocol.

Relevant files:

- `src/browser/**` — browser service/session implementation.
- `src/tool/browserctl/**` — tool schema, input, dispatch, response.
- `src/cli/browserctl/**` — CLI args and command wiring.

Capabilities observed:

- Launch or attach to a Chromium-family browser using CDP.
- Detect Chrome/Chromium/Edge/Brave/Vivaldi.
- Navigate, go back, wait, inspect text/html, evaluate JavaScript, and capture screenshots.
- Manage tabs.
- Upload files into file inputs.
- Click/fill/toggle using selectors.
- Raw viewport mouse clicks and keyboard events through CDP `Input.dispatchMouseEvent` and `Input.dispatchKeyEvent`.
- Humanized pointer movement and typing delays.
- Network inspection and replay using page-resident fetch/XHR/axios helpers.
- Stealth-oriented Chromium launch flags to remove automation banners and reduce `navigator.webdriver` signals.

### Implementation category

This is **browser-process automation**, not **OS desktop automation**.

The tool controls what CDP exposes inside a Chromium browser target. It does not currently own a native desktop cursor, enumerate native app windows, inspect OS accessibility trees, operate Safari/Firefox/native apps, interact with menus outside Chromium, or request/check macOS Screen Recording and Accessibility permissions.

## Difference matrix

| Area | OpenAI in-app browser | OpenAI Computer Use | CodeTether today |
|---|---|---|---|
| Primary scope | Thread browser | Native desktop apps | Chromium-family browser |
| Control API | Browser plugin/in-app browser | macOS Screen Recording + Accessibility | Chrome DevTools Protocol via `chromiumoxide` |
| Auth/session support | No regular profile/cookies/extensions | Can use user's real browser if app approved | Can attach to real CDP profile if browser exposes remote debugging; can also launch managed profile |
| Native app support | No | Yes, app/window/menu GUI | No |
| Browser support | In-app browser | Any approved GUI browser in principle | Chrome/Chromium/Edge/Brave/Vivaldi only |
| Input model | Browser-scoped | OS pointer/keyboard/clipboard/windows | Synthetic CDP input events scoped to page |
| Perception | Rendered page and screenshots | Screen capture and GUI state | DOM text/html, JS eval, screenshots from CDP |
| Window management | App-contained | Native windows/apps | CDP tabs/pages only |
| Permission model | Allowed/blocked websites | App approvals + OS privacy permissions | Tool availability; no app/site permission registry specific to browserctl |
| Safety boundaries | Browser/site allowlist | App allowlist, sensitive-action prompts, cannot automate terminal/Codex/admin prompts | General tool/sandbox policy; browser-specific deny/allow controls are not first-class |
| Parallel GUI work | App claims own cursor/background Mac agents | Yes per announcement | One managed browser service/session model; not OS-level parallel cursors |

## Key gaps

### 1. OS permission integration

We do not check or guide users through macOS Screen Recording or Accessibility permissions. Without those permissions, a Computer Use feature cannot reliably see or control desktop apps.

Bridge:

- Add a `computer_use` tool separate from `browserctl`.
- Add permission probes:
  - macOS Screen Recording availability.
  - macOS Accessibility trust status.
- Return actionable setup guidance when missing.

### 2. Native screen perception

We only capture browser screenshots through CDP. Computer Use needs screen capture of app windows or displays.

Bridge:

- Add a screen adapter abstraction.
- macOS adapter can use native APIs through a small helper binary or crate binding.
- Capture either full display, selected app window, or selected region.
- Ensure screenshots are labeled as sensitive context in logs/UI.

### 3. Native input and window control

Our pointer and keyboard input are CDP events, not OS events. They cannot operate non-browser apps or browser chrome reliably.

Bridge:

- Add an input adapter abstraction with methods like `click`, `type_text`, `press_key`, `hotkey`, `scroll`, and `drag`.
- macOS implementation should use Accessibility/CoreGraphics-level events where allowed.
- Keep CDP input in `browserctl`; do not overload it with OS semantics.

### 4. App discovery and allowlisting

OpenAI uses app-level approvals and an Always Allow list. We do not have a browser/app approval store for GUI targets.

Bridge:

- Add target model: app bundle id, process id, window id/title, display id.
- Add allowlist policy keyed by app identity.
- Require approval before first use of an app.
- Deny terminal apps, CodeTether/Codex-like self-control surfaces, security settings, and admin prompts by default.

### 5. Accessibility tree inspection

Computer Use benefits from OS accessibility metadata: roles, labels, values, focused element, buttons, menus, and coordinates. CodeTether currently uses DOM and browser snapshots only.

Bridge:

- Add optional accessibility tree snapshots for native apps.
- Normalize output into a compact `GuiSnapshot` with visible text, controls, bounds, role, enabled/focused state, and app/window metadata.
- Use screenshots as visual backup when accessibility metadata is sparse.

### 6. Browser chrome and non-CDP browsers

CDP cannot control Safari, Firefox, browser toolbar UI, extensions, native permission dialogs, or signed-in regular browser flows unless the browser is launched with remote debugging and exposes the target page.

Bridge:

- Keep `browserctl` as preferred for structured web testing.
- Use `computer_use` only when the user explicitly needs regular-browser state, browser chrome, extensions, or non-Chromium browsers.
- Add a routing heuristic: structured browser first, OS GUI second.

### 7. Safety and sensitive actions

OpenAI calls out sensitive/disruptive action prompts and limitations around administrator/security/privacy prompts. CodeTether browserctl lacks a GUI-specific policy layer.

Bridge:

- Add GUI action classes: read-only, local navigation, text entry, external submit, payment/security/account change, file/system change.
- Require escalating approvals for higher-risk classes.
- Block or require explicit human takeover for passwords, 2FA, payments, admin prompts, security/privacy settings, and terminal/Codetether self-control.

### 8. Background parallelism

OpenAI claims multiple agents can work on Mac in parallel with their own cursor without interfering with the user. CodeTether can run subagents and browser sessions, but not independent native GUI cursors.

Bridge:

- First milestone: one OS GUI controller at a time with a visible activity indicator.
- Later milestone: app/window-scoped sessions with serialized native input and collision detection.
- Treat “own cursor” as product-level polish after safe single-controller semantics work.

## Proposed architecture

```text
Agent tool layer
├── browserctl          # existing structured CDP browser automation
└── computer_use        # new OS GUI automation tool

computer_use
├── policy             # app allowlist, blocked apps, action risk classes
├── permissions        # OS permission probes and setup guidance
├── target             # app/window/display discovery and selection
├── perception         # screenshot + accessibility snapshots
├── input              # native click/type/key/scroll/drag
└── audit              # screenshot/action metadata and approval trail

OS adapters
├── macos              # Screen Recording + Accessibility implementation
├── windows            # future UI Automation + Graphics Capture
└── linux              # future X11/Wayland-specific support
```

## Tool API sketch

A future `computer_use` tool should be intentionally separate from `browserctl`:

```json
{
  "action": "snapshot",
  "app": "Chrome",
  "window_title_contains": "Checkout"
}
```

```json
{
  "action": "click",
  "target": { "app": "System Settings", "role": "button", "label": "Privacy & Security" }
}
```

```json
{
  "action": "type_text",
  "app": "Simulator",
  "text": "hello@example.com"
}
```

Recommended initial actions:

- `status` — OS support and permission status.
- `list_apps` — visible controllable apps/windows.
- `request_app` — approval workflow for target app.
- `snapshot` — screenshot/accessibility snapshot.
- `click` — coordinate or accessibility-node click.
- `type_text` — native typing.
- `press_key` / `hotkey` — keyboard shortcuts.
- `scroll` — native scroll.
- `stop` — release control and clear active target.

## Milestone plan

### M1: Research spike and boundaries

- Confirm macOS API choices and Rust integration strategy.
- Define blocked app list and sensitive-action classifier.
- Decide where GUI approvals live relative to existing tool approvals/audit.

### M2: Read-only macOS perception

- Implement `computer_use status`.
- Implement app/window listing.
- Implement screenshot capture for selected window/display.
- Add audit records for screen captures.

### M3: Accessibility snapshot

- Add native accessibility tree extraction.
- Normalize into compact snapshot output.
- Add target resolution by app/window/role/label/bounds.

### M4: Native input MVP

- Implement click, type_text, press_key, scroll.
- Gate every app by allowlist approval.
- Block self-control, terminal apps, admin prompts, and security prompts.

### M5: Browser bridge routing

- Add guidance/routing: use `browserctl` for localhost/CDP-capable structured browser tasks; use `computer_use` for regular browser profile, non-Chromium, browser chrome, native apps, simulators, or GUI-only bugs.
- Add docs explaining tradeoffs and safety guidance.

### M6: Cross-platform expansion

- Windows adapter using UI Automation and Windows Graphics Capture.
- Linux adapter only after deciding X11 vs Wayland support boundaries.

## Immediate recommendation

Do not replace `browserctl`. It is stronger than Computer Use for deterministic web automation, DOM inspection, JavaScript evaluation, network replay, and repeatable local app checks. Instead, add `computer_use` as a higher-risk bridge layer for tasks that need the actual desktop.

The product distinction should be explicit:

- **Use `browserctl` when structure exists.** It is deterministic and auditable.
- **Use `computer_use` when only the GUI exists.** It is flexible but sensitive and should require narrower scope and stronger approvals.
