from __future__ import annotations

from playwright.async_api import async_playwright

from .paths import default_browser_path, profile_dir


class BrowserSessionLifecycle:
    async def start(self, headless: bool, executable_path: str | None) -> dict:
        if self.context:
            return await self.snapshot()
        self.playwright = await async_playwright().start()
        browser_path = executable_path or default_browser_path()
        self.context = await self.playwright.chromium.launch_persistent_context(
            str(profile_dir()), executable_path=browser_path, headless=headless, args=["--no-sandbox"]
        )
        self.context.on("page", self._attach_page)
        for page in self.context.pages:
            self._attach_page(page)
        self.page = self.context.pages[0] if self.context.pages else await self.context.new_page()
        self._attach_page(self.page)
        return await self.snapshot()

    async def stop(self) -> dict:
        if self.context:
            await self.context.close()
        if self.playwright:
            await self.playwright.stop()
        self.playwright = None
        self.context = None
        self.page = None
        return {"ok": True}

    async def snapshot(self) -> dict:
        page = await self.current_page()
        return {"url": page.url, "title": await page.title(), "text": (await page.locator("body").inner_text())[:4000]}