from __future__ import annotations


class BrowserSessionTabs:
    async def list_tabs(self) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        current = await self.current_page()
        tabs = [{"index": i, "url": page.url, "title": await page.title(), "active": page == current} for i, page in enumerate(self.context.pages)]
        return {"tabs": tabs}

    async def select_tab(self, index: int) -> dict:
        if not self.context or index < 0 or index >= len(self.context.pages):
            raise RuntimeError(f"tab index out of range: {index}")
        self.page = self.context.pages[index]
        await self.page.bring_to_front()
        return await self.snapshot()

    async def new_tab(self, url: str | None = None) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        self.page = await self.context.new_page()
        self._attach_page(self.page)
        if url:
            await self.page.goto(url, wait_until="domcontentloaded")
        return await self.snapshot()

    async def close_tab(self, index: int | None = None) -> dict:
        if not self.context or not self.context.pages:
            raise RuntimeError("no tabs to close")
        page = await self.current_page() if index is None else self.context.pages[index]
        await page.close()
        self.page = self.context.pages[0] if self.context.pages else None
        if self.page:
            await self.page.bring_to_front()
            return await self.snapshot()
        return {"ok": True, "tabs": []}