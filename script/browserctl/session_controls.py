from __future__ import annotations

NATIVE_FILL = "(el, value) => { const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype; const setter = Object.getOwnPropertyDescriptor(proto, 'value')?.set; if (!setter) throw new Error('native value setter not found'); el.focus(); setter.call(el, value); el.dispatchEvent(new InputEvent('input', { bubbles: true, data: value, inputType: 'insertText' })); el.dispatchEvent(new Event('change', { bubbles: true })); el.dispatchEvent(new Event('blur', { bubbles: true })); }"
ACTIVE_TOKENS = ["selected", "active", "bg-sky-500/20", "text-sky-400"]


class BrowserSessionControls:
    async def click_text(self, text: str, timeout_ms: int = 5_000, selector: str | None = None, frame_selector: str | None = None, exact: bool = True, index: int = 0) -> dict:
        scope = await self.scope(frame_selector)
        locator = scope.locator(selector).first.get_by_text(text, exact=exact) if selector else scope.get_by_text(text, exact=exact)
        await locator.nth(index).click(timeout=timeout_ms)
        return {"ok": True}

    async def fill_native(self, selector: str, value: str, frame_selector: str | None = None) -> dict:
        locator = await self.first_locator(selector, frame_selector)
        await locator.wait_for()
        await locator.scroll_into_view_if_needed()
        await locator.evaluate(NATIVE_FILL, value)
        return {"ok": True}

    async def set_toggle(self, group_selector: str, text: str, timeout_ms: int = 5_000, frame_selector: str | None = None) -> dict:
        group = await self.first_locator(group_selector, frame_selector)
        await group.wait_for(timeout=timeout_ms)
        await group.scroll_into_view_if_needed()
        button = group.get_by_role("button", name=text, exact=True).first
        await button.scroll_into_view_if_needed()
        await button.click(timeout=timeout_ms)
        classes = await button.get_attribute("class") or ""
        return {"ok": True, "selected": await button.get_attribute("aria-pressed"), "active_class": any(token in classes for token in ACTIVE_TOKENS)}