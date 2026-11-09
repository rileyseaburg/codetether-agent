from __future__ import annotations

from playwright.async_api import BrowserContext, Page, Playwright


class BrowserSessionCore:
    def __init__(self) -> None:
        self.playwright: Playwright | None = None
        self.context: BrowserContext | None = None
        self.page: Page | None = None
        self.console_messages: list[str] = []

    def _attach_page(self, page: Page) -> None:
        page.on("console", self._capture_console)

    def _capture_console(self, message) -> None:
        self.console_messages.append(f"{message.type}: {message.text}")
        self.console_messages[:] = self.console_messages[-100:]

    async def current_page(self) -> Page:
        if not self.page:
            raise RuntimeError("browser not started")
        return self.page