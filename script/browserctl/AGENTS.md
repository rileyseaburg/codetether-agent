# AGENTS.md

Instructions for AI coding agents working in `script/browserctl`.

## Scope

These instructions apply to the browser control shim under `script/browserctl/`.

## Purpose

This package is a lightweight Playwright-backed browser control service used for:
- remote browser automation
- manual remote-control login flows
- DOM inspection through JavaScript evaluation
- robust interaction with controlled React and MUI forms
- annotation workflows that require reliable field filling, toggle selection, and iframe inspection

Prefer improving reliability for real-world operator workflows over adding broad abstractions.

## Development Priorities

1. **JS console workflows first.** If an interaction is flaky through generic selector clicks, add or improve a JavaScript or DOM-aware helper.
2. **Controlled form support matters.** React and MUI inputs often require native setters plus `input`, `change`, and sometimes `blur` events.
3. **Verification over hope.** New helpers should make it easy to confirm the UI state actually changed, for example by returning `aria-pressed`, field values, or other observable state.
4. **Keep endpoints small and composable.** Prefer a few focused endpoints over one large opaque automation command.
5. **Do not weaken auth.** Keep bearer-token support intact.

## Preferred Patterns

- Prefer `page.evaluate(...)` or locator-aware helpers when generic click simulation is unreliable.
- For text fields in modern web apps, prefer native setter based filling over raw `.fill(...)` when controlled components are involved.
- For toggle groups, target the group container and the visible button text, then return the resulting selected state.
- When possible, return structured JSON that helps downstream automation verify success.
- Keep async support for page-side scripts.
- Favor startup defaults that reduce operator friction, for example auto-starting the browser session when the server boots.

## When Editing

If you add a new endpoint or helper, also update:
- `README.md` in this directory
- `client.py` if the helper is useful from the CLI
- request models in `models.py`

## Validation

At minimum, run:

```bash
python3 -m py_compile script/browserctl/*.py
```

If you changed behavior substantially, include at least one concrete usage example in the local README.
