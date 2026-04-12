from __future__ import annotations

import os

from fastapi import Depends, FastAPI, HTTPException, Request

from script.browserctl.auth import require_token
from script.browserctl.browser import BrowserSession
from script.browserctl.models import (
    ConsoleRequest,
    EvalRequest,
    FillNativeRequest,
    FillRequest,
    GotoRequest,
    PressRequest,
    ScreenshotRequest,
    SelectorActionRequest,
    StartRequest,
    TypeRequest,
    SelectorRequest,
    PointRequest,
    KeyRequest,
    KeyboardTypeRequest,
    TabSelectRequest,
    WaitRequest,
)
from script.browserctl.ui_routes import (
    image,
    keyboard_press,
    keyboard_type,
    reload_page,
    remote_html,
    viewport,
    mouse_click,
)

app = FastAPI(title="browserctl")
session = BrowserSession()


@app.on_event("startup")
async def startup_event() -> None:
    if os.environ.get("BROWSERCTL_AUTO_START", "1") != "1":
        return
    if session.context:
        return
    headless = os.environ.get("BROWSERCTL_HEADLESS", "0") == "1"
    executable_path = os.environ.get("BROWSERCTL_EXECUTABLE_PATH") or None
    try:
        await session.start(headless=headless, executable_path=executable_path)
    except Exception:
        # Keep the service available even if the browser fails to start.
        pass


def wrap_error(exc: Exception) -> None:
    raise HTTPException(status_code=400, detail=str(exc)) from exc


@app.get("/health")
async def health() -> dict:
    return {"ok": True}


@app.post("/start")
async def start(req: StartRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.start(req.headless, req.executable_path)
    except Exception as exc:
        wrap_error(exc)


@app.post("/stop")
async def stop(_: None = Depends(require_token)) -> dict:
    return await session.stop()


@app.get("/snapshot")
async def snapshot(_: None = Depends(require_token)) -> dict:
    try:
        return await session.snapshot()
    except Exception as exc:
        wrap_error(exc)


@app.get("/console")
async def console(_: None = Depends(require_token)) -> dict:
    return {"messages": session.console_messages}


@app.get("/remote")
async def remote(request: Request) -> object:
    require_token(request)
    return remote_html()


@app.get("/image")
async def image_view(_: None = Depends(require_token)) -> object:
    return await image(session)


@app.get("/viewport")
async def viewport_view(_: None = Depends(require_token)) -> dict:
    return await viewport(session)


@app.post("/goto")
async def goto(req: GotoRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.goto(req.url, req.wait_until)
    except Exception as exc:
        wrap_error(exc)


@app.post("/back")
async def back(_: None = Depends(require_token)) -> dict:
    try:
        return await session.back()
    except Exception as exc:
        wrap_error(exc)


@app.post("/click")
async def click(req: SelectorRequest, _: None = Depends(require_token)) -> dict:
    if not req.selector:
        raise HTTPException(status_code=400, detail="selector is required")
    try:
        return await session.click(req.selector, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/fill")
async def fill(req: FillRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.fill(req.selector, req.value, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/type")
async def type_text(req: TypeRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.type(req.selector, req.text, req.delay_ms, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/press")
async def press(req: PressRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.press(req.selector, req.key, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/text")
async def text(req: SelectorRequest | None = None, _: None = Depends(require_token)) -> dict:
    selector = None if req is None else req.selector
    frame_selector = None if req is None else req.frame_selector
    try:
        return await session.text(selector, frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/html")
async def html(req: SelectorRequest | None = None, _: None = Depends(require_token)) -> dict:
    selector = None if req is None else req.selector
    frame_selector = None if req is None else req.frame_selector
    try:
        return await session.html(selector, frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/eval")
async def eval_js(req: EvalRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.eval(req.expression, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/console/eval")
async def console_eval(req: ConsoleRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.console_eval(req.script, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/click/text")
async def click_text(req: SelectorActionRequest, _: None = Depends(require_token)) -> dict:
    if not req.text:
        raise HTTPException(status_code=400, detail="text is required")
    try:
        return await session.click_text(
            req.text,
            req.timeout_ms,
            req.selector,
            req.frame_selector,
            req.exact,
            req.index,
        )
    except Exception as exc:
        wrap_error(exc)


@app.post("/fill/native")
async def fill_native(req: FillNativeRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.fill_native(req.selector, req.value, req.frame_selector)
    except Exception as exc:
        wrap_error(exc)


@app.post("/toggle")
async def set_toggle(req: SelectorActionRequest, _: None = Depends(require_token)) -> dict:
    if not req.selector:
        raise HTTPException(status_code=400, detail="selector is required")
    if not req.text:
        raise HTTPException(status_code=400, detail="text is required")
    try:
        return await session.set_toggle(
            req.selector,
            req.text,
            req.timeout_ms,
            req.frame_selector,
        )
    except Exception as exc:
        wrap_error(exc)


@app.post("/screenshot")
async def screenshot(req: ScreenshotRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.screenshot(
            req.path,
            req.full_page,
            req.selector,
            req.frame_selector,
        )
    except Exception as exc:
        wrap_error(exc)


@app.post("/wait")
async def wait(req: WaitRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.wait(
            text=req.text,
            text_gone=req.text_gone,
            url_contains=req.url_contains,
            selector=req.selector,
            frame_selector=req.frame_selector,
            state=req.state,
            timeout_ms=req.timeout_ms,
        )
    except Exception as exc:
        wrap_error(exc)


@app.post("/mouse/click")
async def click_at(req: PointRequest, _: None = Depends(require_token)) -> dict:
    return await mouse_click(session, req)


@app.post("/keyboard/type")
async def type_global(req: KeyboardTypeRequest, _: None = Depends(require_token)) -> dict:
    return await keyboard_type(session, req)


@app.post("/keyboard/press")
async def press_global(req: KeyRequest, _: None = Depends(require_token)) -> dict:
    return await keyboard_press(session, req)


@app.post("/reload")
async def reload_current(_: None = Depends(require_token)) -> dict:
    return await reload_page(session)


@app.get("/tabs")
async def list_tabs(_: None = Depends(require_token)) -> dict:
    try:
        return await session.list_tabs()
    except Exception as exc:
        wrap_error(exc)


@app.post("/tabs/select")
async def select_tab(req: TabSelectRequest, _: None = Depends(require_token)) -> dict:
    try:
        return await session.select_tab(req.index)
    except Exception as exc:
        wrap_error(exc)


@app.post("/tabs/new")
async def new_tab(req: GotoRequest | None = None, _: None = Depends(require_token)) -> dict:
    try:
        url = None if req is None else req.url
        return await session.new_tab(url)
    except Exception as exc:
        wrap_error(exc)


@app.post("/tabs/close")
async def close_tab(req: TabSelectRequest | None = None, _: None = Depends(require_token)) -> dict:
    try:
        index = None if req is None else req.index
        return await session.close_tab(index)
    except Exception as exc:
        wrap_error(exc)
