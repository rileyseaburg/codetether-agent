from __future__ import annotations

from fastapi import HTTPException
from fastapi.responses import FileResponse, HTMLResponse, Response

from script.browserctl.models import KeyRequest, KeyboardTypeRequest, PointRequest
from script.browserctl.paths import project_root


def remote_html() -> HTMLResponse:
    path = project_root() / "script" / "browserctl" / "remote.html"
    return HTMLResponse(path.read_text())


def wrap_error(exc: Exception) -> None:
    raise HTTPException(status_code=400, detail=str(exc)) from exc


async def viewport(session) -> dict:
    try:
        return await session.viewport()
    except Exception as exc:
        wrap_error(exc)


async def image(session) -> Response:
    try:
        return Response(await session.image_bytes(), media_type="image/png")
    except Exception as exc:
        wrap_error(exc)


async def mouse_click(session, req: PointRequest) -> dict:
    try:
        return await session.mouse_click(req.x, req.y)
    except Exception as exc:
        wrap_error(exc)


async def keyboard_type(session, req: KeyboardTypeRequest) -> dict:
    try:
        return await session.keyboard_type(req.text)
    except Exception as exc:
        wrap_error(exc)


async def keyboard_press(session, req: KeyRequest) -> dict:
    try:
        return await session.keyboard_press(req.key)
    except Exception as exc:
        wrap_error(exc)


async def reload_page(session) -> dict:
    try:
        return await session.reload()
    except Exception as exc:
        wrap_error(exc)
