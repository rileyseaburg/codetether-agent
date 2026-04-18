from __future__ import annotations

from playwright.async_api import FrameLocator, Locator, Page


class BrowserSessionScope:
    async def scope(self, frame_selector: str | None = None) -> Page | FrameLocator:
        page = await self.current_page()
        return page if not frame_selector else page.frame_locator(frame_selector).first

    async def first_locator(self, selector: str, frame_selector: str | None = None) -> Locator:
        return (await self.scope(frame_selector)).locator(selector).first