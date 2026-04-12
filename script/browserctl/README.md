# browserctl

Small local browser service for CodeTether development.

## Start

```bash
./script/browserctl.sh
```

With auth:

```bash
BROWSERCTL_TOKEN=change-me ./script/browserctl.sh
```

## Use

```bash
python3 -m script.browserctl.client start
python3 -m script.browserctl.client goto https://github.com
python3 -m script.browserctl.client eval "document.title"
python3 -m script.browserctl.client console-eval "return { title: document.title, url: location.href };"
python3 -m script.browserctl.client click-text "Sign in"
python3 -m script.browserctl.client fill-native '#root_overall_assessment_overall_rationale' 'Detailed rationale here'
python3 -m script.browserctl.client toggle '#root_overall_assessment_overall_rating' '1'
python3 -m script.browserctl.client screenshot /tmp/github.png
python3 -m script.browserctl.client console
```

## Remote Login UI

Open this in your normal browser:

```text
http://127.0.0.1:4477/remote
```

It shows a live screenshot of the controlled browser and lets you click,
type, press keys, reload, and navigate. Use it for one-time manual login flows.

When `BROWSERCTL_TOKEN` is set:

```text
http://127.0.0.1:4477/remote?token=change-me
```

## Local Companion For Passkeys

Run the browser on your local machine so WebAuthn/passkeys can use your phone:

```bash
BROWSERCTL_TOKEN=change-me BROWSERCTL_HOST=127.0.0.1 python3 -m script.browserctl
```

By default, the service now auto-starts a browser session on launch. You can control that behavior with:

```bash
# Disable browser auto-start
BROWSERCTL_AUTO_START=0 python3 -m script.browserctl

# Start headless automatically instead of headed
BROWSERCTL_HEADLESS=1 python3 -m script.browserctl

# Force a specific browser binary
BROWSERCTL_EXECUTABLE_PATH=/path/to/chrome python3 -m script.browserctl
```

Then reverse-tunnel it to the remote host:

```bash
ssh -R 4478:127.0.0.1:4477 riley@192.168.50.101
```

From the remote host, use:

```bash
BROWSERCTL_BASE=http://127.0.0.1:4478 BROWSERCTL_TOKEN=change-me \
python3 -m script.browserctl.client goto https://github.com
```

## Endpoints

Core endpoints:

- `POST /start`
- `POST /goto`
- `POST /click`
- `POST /fill`
- `POST /type`
- `POST /press`
- `POST /text`
- `POST /html`
- `POST /eval`
- `POST /console/eval`
- `POST /click/text`
- `POST /fill/native`
- `POST /toggle`
- `POST /screenshot`
- `GET /snapshot`
- `GET /console`
- `GET /health`
- `GET /tabs`
- `POST /tabs/select`
- `POST /tabs/new`
- `POST /tabs/close`

## Why the new helpers exist

Modern React and MUI interfaces often ignore naive automation that only calls a plain click or assigns `.value` directly. The new helpers are intended for the cases that matter in day to day annotation and browser automation work:

- `POST /console/eval` runs async page-side JavaScript like a lightweight JS console.
- `POST /fill/native` uses the native value setter and dispatches `input`, `change`, and `blur`, which is much more reliable for controlled components.
- `POST /toggle` targets a toggle-group container and a visible option label, then returns the selected state.
- `POST /click/text` is useful when visible button text is more stable than a CSS selector.

## Example workflow for controlled forms

```bash
# Inspect exact field ids and names
python3 -m script.browserctl.client console-eval \
  "return Array.from(document.querySelectorAll('textarea')).map((t, i) => ({ i, id: t.id, name: t.name, aria: t.getAttribute('aria-label') }));"

# Fill a controlled textarea
python3 -m script.browserctl.client fill-native \
  '#root_overall_assessment_overall_rationale' \
  'The app fails most core requirements because the main add flow is broken.'

# Set a MUI toggle group by container id and visible button text
python3 -m script.browserctl.client toggle '#root_overall_assessment_overall_rating' '1'
python3 -m script.browserctl.client toggle '#root_overall_assessment_rubrics_sufficient' 'Yes, the rubrics were sufficient'
```
