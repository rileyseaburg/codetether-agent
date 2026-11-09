from __future__ import annotations


class BrowserSessionNavigation:
    async def goto(self, url: str, wait_until: str) -> dict:
        page = await self.current_page()
        await page.goto(url, wait_until=wait_until)
        return await self.snapshot()

    async def reload(self) -> dict:
        await (await self.current_page()).reload(wait_until="domcontentloaded")
        return await self.snapshot()

    async def back(self) -> dict:
        await (await self.current_page()).go_back(wait_until="domcontentloaded")
        return await self.snapshot()

    async def wait(self, *, text: str | None = None, text_gone: str | None = None, url_contains: str | None = None, selector: str | None = None, frame_selector: str | None = None, state: str = "visible", timeout_ms: int = 5_000) -> dict:
        page = await self.current_page()
        scope = await self.scope(frame_selector)
        if url_contains:
            await page.wait_for_url(f"**{url_contains}**", timeout=timeout_ms)
        if selector:
            await scope.locator(selector).first.wait_for(timeout=timeout_ms, state=state)
        if text:
            await scope.get_by_text(text, exact=False).first.wait_for(timeout=timeout_ms, state=state)
        if text_gone:
            await scope.get_by_text(text_gone, exact=False).first.wait_for(timeout=timeout_ms, state="hidden")
        return await self.snapshot()