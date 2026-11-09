from __future__ import annotations


class BrowserSessionInput:
    async def mouse_click(self, x: float, y: float) -> dict:
        await (await self.current_page()).mouse.click(x, y)
        return {"ok": True}

    async def keyboard_type(self, text: str) -> dict:
        await (await self.current_page()).keyboard.type(text)
        return {"ok": True}

    async def keyboard_press(self, key: str) -> dict:
        await (await self.current_page()).keyboard.press(key)
        return {"ok": True}