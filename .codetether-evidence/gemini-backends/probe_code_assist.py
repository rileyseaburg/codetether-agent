#!/usr/bin/env python3
"""Probe the OAuth-authenticated Code Assist backend the official CLI uses.

Verified against google-gemini/gemini-cli upstream source:
  packages/core/src/code_assist/server.ts
    CODE_ASSIST_ENDPOINT   = https://cloudcode-pa.googleapis.com
    CODE_ASSIST_API_VERSION = v1internal
    methods: loadCodeAssist, onboardUser, generateContent,
             streamGenerateContent (?alt=sse), countTokens
  packages/core/src/code_assist/oauth2.ts
    OAUTH_SCOPE = cloud-platform, userinfo.email, userinfo.profile

Unlike the cookie transport, this speaks documented-shape JSON: tool calls are
real `functionCall` parts and responses carry `usageMetadata` token counts.

Credentials: an OAuth access token. Never hardcoded. Resolution order:
  1. GEMINI_CODE_ASSIST_ACCESS_TOKEN
  2. ~/.gemini/oauth_creds.json  (written by `gemini` login)
  3. gcloud auth print-access-token

Usage:
    probe_code_assist.py "prompt text" [--json] [--stream]
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import subprocess
import sys

import requests

ENDPOINT = "https://cloudcode-pa.googleapis.com"
API_VERSION = "v1internal"
OAUTH_CLIENT_ID = (
    "681255809395-oo8ft2oprdrnp9e3aqf6av3hmdib135j.apps.googleusercontent.com"
)
OAUTH_SCOPES = [
    "https://www.googleapis.com/auth/cloud-platform",
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/userinfo.profile",
]
DEFAULT_MODEL = "gemini-2.5-pro"


def access_token() -> tuple[str, str]:
    """Return (token, source) without printing the secret."""
    env = os.environ.get("GEMINI_CODE_ASSIST_ACCESS_TOKEN")
    if env:
        return env.strip(), "env GEMINI_CODE_ASSIST_ACCESS_TOKEN"

    creds = pathlib.Path.home() / ".gemini" / "oauth_creds.json"
    if creds.is_file():
        payload = json.loads(creds.read_text())
        token = payload.get("access_token")
        if token:
            return token, str(creds)

    probe = subprocess.run(
        ["gcloud", "auth", "print-access-token"],
        capture_output=True, text=True,
    )
    if probe.returncode == 0 and probe.stdout.strip():
        return probe.stdout.strip(), "gcloud auth print-access-token"

    raise RuntimeError(
        "no Code Assist OAuth token; run `gemini` login, set "
        "GEMINI_CODE_ASSIST_ACCESS_TOKEN, or authenticate gcloud"
    )


def method_url(method: str) -> str:
    return f"{ENDPOINT}/{API_VERSION}:{method}"


def tool_declaration() -> dict:
    """A real function declaration, the CLI's native tool mechanism."""
    return {
        "functionDeclarations": [
            {
                "name": "read_file",
                "description": "Read a UTF-8 file from the workspace.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Absolute path to read.",
                        }
                    },
                    "required": ["path"],
                },
            }
        ]
    }


def load_project(token: str) -> dict:
    """Call loadCodeAssist, which resolves tier and project for the account."""
    response = requests.post(
        method_url("loadCodeAssist"),
        headers={"Authorization": f"Bearer {token}",
                 "Content-Type": "application/json"},
        json={"metadata": {"pluginType": "GEMINI"}},
        timeout=60,
    )
    return {
        "http_status": response.status_code,
        "body": safe_json(response),
        "error_detail": None if response.ok else str(safe_json(response))[:600],
    }


def safe_json(response: requests.Response) -> object:
    try:
        return response.json()
    except ValueError:
        return response.text[:2000]


def generate(token: str, prompt: str, project: str | None, model: str) -> dict:
    """One non-streaming generateContent turn with a real tool declared."""
    request: dict = {
        "model": model,
        "request": {
            "contents": [{"role": "user", "parts": [{"text": prompt}]}],
            "tools": [tool_declaration()],
        },
    }
    if project:
        request["project"] = project

    response = requests.post(
        method_url("generateContent"),
        headers={"Authorization": f"Bearer {token}",
                 "Content-Type": "application/json"},
        json=request,
        timeout=180,
    )
    body = safe_json(response)
    return {
        "http_status": response.status_code,
        "answer_text": first_text(body),
        "function_calls": function_calls(body),
        "usage_metadata": usage(body),
        "raw": body,
    }


def _parts(body: object) -> list[dict]:
    if not isinstance(body, dict):
        return []
    inner = body.get("response", body)
    candidates = inner.get("candidates") if isinstance(inner, dict) else None
    if not candidates:
        return []
    content = candidates[0].get("content", {})
    parts = content.get("parts", [])
    return [p for p in parts if isinstance(p, dict)]


def first_text(body: object) -> str | None:
    texts = [p["text"] for p in _parts(body) if isinstance(p.get("text"), str)]
    return "\n".join(texts) if texts else None


def function_calls(body: object) -> list[dict]:
    """Structured tool calls: named, typed, and impossible to forge as prose."""
    return [p["functionCall"] for p in _parts(body) if "functionCall" in p]


def usage(body: object) -> dict | None:
    if not isinstance(body, dict):
        return None
    inner = body.get("response", body)
    return inner.get("usageMetadata") if isinstance(inner, dict) else None


def probe(prompt: str, model: str) -> dict:
    token, source = access_token()
    onboarding = load_project(token)
    body = onboarding.get("body")
    project = body.get("cloudaicompanionProject") if isinstance(body, dict) else None

    turn = generate(token, prompt, project, model)
    return {
        "backend": "cloudcode-pa.googleapis.com Code Assist (OAuth)",
        "endpoint": method_url("generateContent"),
        "api_version": API_VERSION,
        "auth": f"OAuth Bearer token from {source}",
        "oauth_client_id": OAUTH_CLIENT_ID,
        "oauth_scopes": OAUTH_SCOPES,
        "model_selector": f"request.model = {model}",
        "load_code_assist_status": onboarding["http_status"],
        "resolved_project": project,
        "http_status": turn["http_status"],
        "answer_text": turn["answer_text"],
        "structured_tool_calls": bool(turn["function_calls"]),
        "function_calls": turn["function_calls"],
        "reported_token_usage": turn["usage_metadata"] is not None,
        "usage_metadata": turn["usage_metadata"],
        "notes": [
            "answer read from candidates[0].content.parts[].text by name",
            "tool calls arrive as typed functionCall parts",
            "usageMetadata carries real prompt/candidate token counts",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("prompt")
    parser.add_argument("--model", default=DEFAULT_MODEL)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    try:
        result = probe(args.prompt, args.model)
    except Exception as error:
        result = {"backend": "cloudcode-pa.googleapis.com Code Assist (OAuth)",
                  "error": f"{type(error).__name__}: {error}"}

    print(json.dumps(result, indent=2) if args.json else result)
    return 0 if "error" not in result else 1


if __name__ == "__main__":
    sys.exit(main())