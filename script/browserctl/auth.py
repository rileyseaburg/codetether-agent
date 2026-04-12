from __future__ import annotations

import os

from fastapi import HTTPException, Request


def expected_token() -> str | None:
    return os.environ.get("BROWSERCTL_TOKEN")


def require_token(request: Request) -> None:
    token = expected_token()
    if not token:
        return
    auth = request.headers.get("authorization", "")
    header = auth.removeprefix("Bearer ").strip() if auth.startswith("Bearer ") else ""
    query = request.query_params.get("token", "")
    direct = request.headers.get("x-browserctl-token", "")
    if token in {header, query, direct}:
        return
    raise HTTPException(status_code=401, detail="invalid browserctl token")
