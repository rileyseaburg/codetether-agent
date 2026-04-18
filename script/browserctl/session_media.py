from __future__ import annotations

from pathlib import Path


class BrowserSessionMedia:
    async def screenshot(self, path: str, full_page: bool, selector: str | None = None, frame_selector: str | None = None) -> dict:
        target = Path(path).expanduser().resolve()
        target.parent.mkdir(parents=True, exist_ok=True)
        if selector:
            await (await self.first_locator(selector, frame_selector)).screenshot(path=str(target))
            return {"path": str(target), "selector": selector, "frame_selector": frame_selector}
        if frame_selector:
            iframe = (await self.current_page()).locator(frame_selector).first
            await iframe.wait_for()
            await iframe.screenshot(path=str(target))
            return {"path": str(target), "frame_selector": frame_selector}
        await (await self.current_page()).screenshot(path=str(target), full_page=full_page)
        return {"path": str(target)}

    async def image_bytes(self) -> bytes:
        return await (await self.current_page()).screenshot(type="png", full_page=False)

    async def viewport(self) -> dict:
        size = (await self.current_page()).viewport_size
        return size or await (await self.current_page()).evaluate("() => ({ width: window.innerWidth, height: window.innerHeight })")