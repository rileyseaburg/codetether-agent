from __future__ import annotations


class BrowserSessionDom:
    async def click(self, selector: str, frame_selector: str | None = None) -> dict:
        await (await self.first_locator(selector, frame_selector)).click()
        return {"ok": True}

    async def fill(self, selector: str, value: str, frame_selector: str | None = None) -> dict:
        await (await self.first_locator(selector, frame_selector)).fill(value)
        return {"ok": True}

    async def type(self, selector: str, text: str, delay_ms: int, frame_selector: str | None = None) -> dict:
        await (await self.first_locator(selector, frame_selector)).type(text, delay=delay_ms)
        return {"ok": True}

    async def press(self, selector: str, key: str, frame_selector: str | None = None) -> dict:
        await (await self.first_locator(selector, frame_selector)).press(key)
        return {"ok": True}

    async def text(self, selector: str | None = None, frame_selector: str | None = None) -> dict:
        return {"text": (await (await self.first_locator(selector or "body", frame_selector)).inner_text())[:20000]}

    async def html(self, selector: str | None = None, frame_selector: str | None = None) -> dict:
        page = await self.current_page()
        if selector:
            return {"html": await (await self.first_locator(selector, frame_selector)).inner_html()}
        if frame_selector:
            return {"html": await (await self.first_locator("body", frame_selector)).inner_html()}
        return {"html": await page.content()}