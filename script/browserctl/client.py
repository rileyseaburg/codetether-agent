from __future__ import annotations

import argparse
import json
import os
import urllib.request


def _headers(token: str | None = None) -> dict[str, str]:
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def post(base: str, path: str, payload: dict | None = None, token: str | None = None) -> None:
    url = f"{base.rstrip('/')}{path}"
    data = None if payload is None else json.dumps(payload).encode()
    req = urllib.request.Request(url, data=data, headers=_headers(token))
    with urllib.request.urlopen(req) as resp:
        print(json.dumps(json.loads(resp.read().decode()), indent=2))


def get(base: str, path: str, token: str | None = None) -> None:
    url = f"{base.rstrip('/')}{path}"
    req = urllib.request.Request(url, headers=_headers(token))
    with urllib.request.urlopen(req) as resp:
        print(json.dumps(json.loads(resp.read().decode()), indent=2))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", default=os.environ.get("BROWSERCTL_BASE", "http://127.0.0.1:4477"))
    parser.add_argument("--token", default=os.environ.get("BROWSERCTL_TOKEN"))
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("health")
    sub.add_parser("snapshot")
    sub.add_parser("console")
    sub.add_parser("stop")
    sub.add_parser("back")
    sub.add_parser("reload")
    sub.add_parser("tabs")
    start = sub.add_parser("start")
    start.add_argument("--headed", action="store_true")
    start.add_argument("--executable-path")
    goto = sub.add_parser("goto")
    goto.add_argument("url")
    click = sub.add_parser("click")
    click.add_argument("selector")
    click.add_argument("--frame-selector")
    click_text = sub.add_parser("click-text")
    click_text.add_argument("text")
    click_text.add_argument("--selector")
    click_text.add_argument("--frame-selector")
    click_text.add_argument("--contains", action="store_true")
    click_text.add_argument("--index", type=int, default=0)
    click_text.add_argument("--timeout-ms", type=int, default=5000)
    fill = sub.add_parser("fill")
    fill.add_argument("selector")
    fill.add_argument("value")
    fill.add_argument("--frame-selector")
    fill_native = sub.add_parser("fill-native")
    fill_native.add_argument("selector")
    fill_native.add_argument("value")
    fill_native.add_argument("--frame-selector")
    type_cmd = sub.add_parser("type")
    type_cmd.add_argument("selector")
    type_cmd.add_argument("text")
    type_cmd.add_argument("--frame-selector")
    type_cmd.add_argument("--delay-ms", type=int, default=0)
    press = sub.add_parser("press")
    press.add_argument("selector")
    press.add_argument("key")
    press.add_argument("--frame-selector")
    text_cmd = sub.add_parser("text")
    text_cmd.add_argument("--selector")
    text_cmd.add_argument("--frame-selector")
    html_cmd = sub.add_parser("html")
    html_cmd.add_argument("--selector")
    html_cmd.add_argument("--frame-selector")
    eval_cmd = sub.add_parser("eval")
    eval_cmd.add_argument("expression")
    eval_cmd.add_argument("--frame-selector")
    console_eval = sub.add_parser("console-eval")
    console_eval.add_argument("script")
    console_eval.add_argument("--frame-selector")
    toggle = sub.add_parser("toggle")
    toggle.add_argument("selector")
    toggle.add_argument("text")
    toggle.add_argument("--frame-selector")
    toggle.add_argument("--timeout-ms", type=int, default=5000)
    shot = sub.add_parser("screenshot")
    shot.add_argument("path")
    shot.add_argument("--selector")
    shot.add_argument("--frame-selector")
    shot.add_argument("--viewport-only", action="store_true")
    wait = sub.add_parser("wait")
    wait.add_argument("--text")
    wait.add_argument("--text-gone")
    wait.add_argument("--url-contains")
    wait.add_argument("--selector")
    wait.add_argument("--frame-selector")
    wait.add_argument("--state", default="visible")
    wait.add_argument("--timeout-ms", type=int, default=5000)
    tabs_select = sub.add_parser("tabs-select")
    tabs_select.add_argument("index", type=int)
    tabs_new = sub.add_parser("tabs-new")
    tabs_new.add_argument("--url")
    tabs_close = sub.add_parser("tabs-close")
    tabs_close.add_argument("--index", type=int)
    args = parser.parse_args()
    if args.cmd == "health":
        return get(args.base, "/health", token=args.token)
    if args.cmd == "snapshot":
        return get(args.base, "/snapshot", token=args.token)
    if args.cmd == "console":
        return get(args.base, "/console", token=args.token)
    if args.cmd == "tabs":
        return get(args.base, "/tabs", token=args.token)
    if args.cmd == "stop":
        return post(args.base, "/stop", {}, args.token)
    if args.cmd == "back":
        return post(args.base, "/back", {}, args.token)
    if args.cmd == "reload":
        return post(args.base, "/reload", {}, args.token)
    if args.cmd == "start":
        return post(args.base, "/start", {"headless": not args.headed, "executable_path": args.executable_path}, args.token)
    if args.cmd == "goto":
        return post(args.base, "/goto", {"url": args.url}, args.token)
    if args.cmd == "click":
        return post(args.base, "/click", {"selector": args.selector, "frame_selector": args.frame_selector}, args.token)
    if args.cmd == "click-text":
        return post(
            args.base,
            "/click/text",
            {
                "text": args.text,
                "selector": args.selector,
                "frame_selector": args.frame_selector,
                "exact": not args.contains,
                "index": args.index,
                "timeout_ms": args.timeout_ms,
            },
            args.token,
        )
    if args.cmd == "fill":
        return post(
            args.base,
            "/fill",
            {"selector": args.selector, "value": args.value, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "fill-native":
        return post(
            args.base,
            "/fill/native",
            {"selector": args.selector, "value": args.value, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "type":
        return post(
            args.base,
            "/type",
            {
                "selector": args.selector,
                "text": args.text,
                "delay_ms": args.delay_ms,
                "frame_selector": args.frame_selector,
            },
            args.token,
        )
    if args.cmd == "press":
        return post(
            args.base,
            "/press",
            {"selector": args.selector, "key": args.key, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "text":
        return post(
            args.base,
            "/text",
            {"selector": args.selector, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "html":
        return post(
            args.base,
            "/html",
            {"selector": args.selector, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "eval":
        return post(
            args.base,
            "/eval",
            {"expression": args.expression, "frame_selector": args.frame_selector},
            args.token,
        )
    if args.cmd == "console-eval":
        payload = {"script": args.script}
        if args.frame_selector:
            payload["frame_selector"] = args.frame_selector
        return post(args.base, "/console/eval", payload, args.token)
    if args.cmd == "toggle":
        return post(
            args.base,
            "/toggle",
            {
                "selector": args.selector,
                "text": args.text,
                "frame_selector": args.frame_selector,
                "timeout_ms": args.timeout_ms,
            },
            args.token,
        )
    if args.cmd == "screenshot":
        return post(
            args.base,
            "/screenshot",
            {
                "path": args.path,
                "full_page": not args.viewport_only,
                "selector": args.selector,
                "frame_selector": args.frame_selector,
            },
            args.token,
        )
    if args.cmd == "wait":
        return post(
            args.base,
            "/wait",
            {
                "text": args.text,
                "text_gone": args.text_gone,
                "url_contains": args.url_contains,
                "selector": args.selector,
                "frame_selector": args.frame_selector,
                "state": args.state,
                "timeout_ms": args.timeout_ms,
            },
            args.token,
        )
    if args.cmd == "tabs-select":
        return post(args.base, "/tabs/select", {"index": args.index}, args.token)
    if args.cmd == "tabs-new":
        payload = {} if args.url is None else {"url": args.url}
        return post(args.base, "/tabs/new", payload, args.token)
    if args.cmd == "tabs-close":
        payload = {} if args.index is None else {"index": args.index}
        return post(args.base, "/tabs/close", payload, args.token)


if __name__ == "__main__":
    main()
