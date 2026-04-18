from __future__ import annotations

FRAME_EVAL = "(el, expression) => el.ownerDocument.defaultView.eval(expression)"
ASYNC_EVAL = "async (script) => { const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor; const fn = new AsyncFunction(script); return await fn(); }"


class BrowserSessionEval:
    async def eval(self, expression: str, frame_selector: str | None = None) -> dict:
        if frame_selector:
            body = await self.first_locator("body", frame_selector)
            return {"result": await body.evaluate(FRAME_EVAL, expression)}
        return {"result": await (await self.current_page()).evaluate(expression)}

    async def console_eval(self, script: str, frame_selector: str | None = None) -> dict:
        page = await self.current_page()
        if frame_selector:
            body = page.frame_locator(frame_selector).first.locator("body")
            return {"result": await body.evaluate(ASYNC_EVAL, script)}
        return {"result": await page.evaluate(ASYNC_EVAL, script)}