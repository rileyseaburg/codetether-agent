from __future__ import annotations

from pydantic import BaseModel


class StartRequest(BaseModel):
    headless: bool = True
    executable_path: str | None = None


class GotoRequest(BaseModel):
    url: str
    wait_until: str = "domcontentloaded"


class SelectorRequest(BaseModel):
    selector: str | None = None
    frame_selector: str | None = None


class FillRequest(SelectorRequest):
    selector: str
    value: str


class TypeRequest(SelectorRequest):
    selector: str
    text: str
    delay_ms: int = 0


class PressRequest(SelectorRequest):
    selector: str
    key: str


class EvalRequest(BaseModel):
    expression: str
    frame_selector: str | None = None


class ConsoleRequest(BaseModel):
    script: str
    frame_selector: str | None = None


class SelectorActionRequest(SelectorRequest):
    text: str | None = None
    timeout_ms: int = 5_000
    exact: bool = True
    index: int = 0


class FillNativeRequest(SelectorRequest):
    selector: str
    value: str


class ScreenshotRequest(BaseModel):
    path: str
    full_page: bool = True
    selector: str | None = None
    frame_selector: str | None = None


class PointRequest(BaseModel):
    x: float
    y: float


class KeyRequest(BaseModel):
    key: str


class KeyboardTypeRequest(BaseModel):
    text: str


class TabSelectRequest(BaseModel):
    index: int


class WaitRequest(BaseModel):
    text: str | None = None
    text_gone: str | None = None
    url_contains: str | None = None
    selector: str | None = None
    frame_selector: str | None = None
    state: str = "visible"
    timeout_ms: int = 5_000
