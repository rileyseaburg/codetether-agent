from __future__ import annotations

from pathlib import Path

from playwright.async_api import BrowserContext, FrameLocator, Locator, Page, Playwright, async_playwright

from script.browserctl.paths import default_browser_path, profile_dir


class BrowserSession:
    def __init__(self) -> None:
        self.playwright: Playwright | None = None
        self.context: BrowserContext | None = None
        self.page: Page | None = None
        self.console_messages: list[str] = []

    def _attach_page(self, page: Page) -> None:
        page.on("console", self._capture_console)

    async def start(self, headless: bool, executable_path: str | None) -> dict:
        if self.context:
            return await self.snapshot()
        self.playwright = await async_playwright().start()
        browser_path = executable_path or default_browser_path()
        self.context = await self.playwright.chromium.launch_persistent_context(
            str(profile_dir()),
            executable_path=browser_path,
            headless=headless,
            args=["--no-sandbox"],
        )
        self.context.on("page", self._attach_page)
        for existing_page in self.context.pages:
            self._attach_page(existing_page)
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

    def _capture_console(self, message) -> None:
        self.console_messages.append(f"{message.type}: {message.text}")
        self.console_messages[:] = self.console_messages[-100:]

    async def current_page(self) -> Page:
        if not self.page:
            raise RuntimeError("browser not started")
        return self.page

    async def scope(self, frame_selector: str | None = None) -> Page | FrameLocator:
        page = await self.current_page()
        if not frame_selector:
            return page
        return page.frame_locator(frame_selector).first

    async def first_locator(self, selector: str, frame_selector: str | None = None) -> Locator:
        return (await self.scope(frame_selector)).locator(selector).first

    async def snapshot(self) -> dict:
        page = await self.current_page()
        text = await page.locator("body").inner_text()
        return {"url": page.url, "title": await page.title(), "text": text[:4000]}

    async def goto(self, url: str, wait_until: str) -> dict:
        page = await self.current_page()
        await page.goto(url, wait_until=wait_until)
        return await self.snapshot()

    async def click(self, selector: str, frame_selector: str | None = None) -> dict:
        await (await self.first_locator(selector, frame_selector)).click()
        return {"ok": True}

    async def fill(
        self,
        selector: str,
        value: str,
        frame_selector: str | None = None,
    ) -> dict:
        await (await self.first_locator(selector, frame_selector)).fill(value)
        return {"ok": True}

    async def type(
        self,
        selector: str,
        text: str,
        delay_ms: int,
        frame_selector: str | None = None,
    ) -> dict:
        await (await self.first_locator(selector, frame_selector)).type(text, delay=delay_ms)
        return {"ok": True}

    async def press(
        self,
        selector: str,
        key: str,
        frame_selector: str | None = None,
    ) -> dict:
        await (await self.first_locator(selector, frame_selector)).press(key)
        return {"ok": True}

    async def text(
        self,
        selector: str | None = None,
        frame_selector: str | None = None,
    ) -> dict:
        locator = await self.first_locator(selector or "body", frame_selector)
        return {"text": (await locator.inner_text())[:20000]}

    async def html(
        self,
        selector: str | None = None,
        frame_selector: str | None = None,
    ) -> dict:
        page = await self.current_page()
        if selector:
            return {"html": await (await self.first_locator(selector, frame_selector)).inner_html()}
        if frame_selector:
            return {"html": await (await self.first_locator("body", frame_selector)).inner_html()}
        return {"html": await page.content()}

    async def eval(self, expression: str, frame_selector: str | None = None) -> dict:
        if frame_selector:
            return {
                "result": await (await self.first_locator("body", frame_selector)).evaluate(
                    "(el, expression) => el.ownerDocument.defaultView.eval(expression)",
                    expression,
                )
            }
        return {"result": await (await self.current_page()).evaluate(expression)}

    async def console_eval(self, script: str, frame_selector: str | None = None) -> dict:
        page = await self.current_page()
        if frame_selector:
            frame = page.frame_locator(frame_selector).first
            return {
                "result": await frame.locator("body").evaluate(
                    """
                    async (el, script) => {
                      const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
                      const fn = new AsyncFunction(script);
                      return await fn();
                    }
                    """,
                    script,
                )
            }
        return {
            "result": await page.evaluate(
                """
                async (script) => {
                  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
                  const fn = new AsyncFunction(script);
                  return await fn();
                }
                """,
                script,
            )
        }

    async def click_text(
        self,
        text: str,
        timeout_ms: int = 5_000,
        selector: str | None = None,
        frame_selector: str | None = None,
        exact: bool = True,
        index: int = 0,
    ) -> dict:
        scope = await self.scope(frame_selector)
        locator = (
            scope.locator(selector).first.get_by_text(text, exact=exact)
            if selector
            else scope.get_by_text(text, exact=exact)
        )
        await locator.nth(index).click(timeout=timeout_ms)
        return {"ok": True}

    async def fill_native(
        self,
        selector: str,
        value: str,
        frame_selector: str | None = None,
    ) -> dict:
        locator = await self.first_locator(selector, frame_selector)
        await locator.wait_for()
        await locator.scroll_into_view_if_needed()
        await locator.evaluate(
            """
            (el, value) => {
              const proto = el instanceof HTMLTextAreaElement
                ? HTMLTextAreaElement.prototype
                : HTMLInputElement.prototype;
              const setter = Object.getOwnPropertyDescriptor(proto, 'value')?.set;
              if (!setter) {
                throw new Error('native value setter not found');
              }
              el.focus();
              setter.call(el, value);
              el.dispatchEvent(new InputEvent('input', { bubbles: true, data: value, inputType: 'insertText' }));
              el.dispatchEvent(new Event('change', { bubbles: true }));
              el.dispatchEvent(new Event('blur', { bubbles: true }));
            }
            """,
            value,
        )
        return {"ok": True}

    async def set_toggle(
        self,
        group_selector: str,
        text: str,
        timeout_ms: int = 5_000,
        frame_selector: str | None = None,
    ) -> dict:
        group = await self.first_locator(group_selector, frame_selector)
        await group.wait_for(timeout=timeout_ms)
        await group.scroll_into_view_if_needed()
        button = group.get_by_role("button", name=text, exact=True).first
        await button.scroll_into_view_if_needed()
        await button.click(timeout=timeout_ms)
        selected = await button.get_attribute("aria-pressed")
        classes = await button.get_attribute("class") or ""
        return {
            "ok": True,
            "selected": selected,
            "active_class": any(token in classes for token in ["selected", "active", "bg-sky-500/20", "text-sky-400"]),
        }

    async def screenshot(
        self,
        path: str,
        full_page: bool,
        selector: str | None = None,
        frame_selector: str | None = None,
    ) -> dict:
        target = Path(path).expanduser().resolve()
        target.parent.mkdir(parents=True, exist_ok=True)
        if selector:
            await (await self.first_locator(selector, frame_selector)).screenshot(path=str(target))
            return {"path": str(target), "selector": selector, "frame_selector": frame_selector}
        if frame_selector:
            page = await self.current_page()
            iframe = page.locator(frame_selector).first
            await iframe.wait_for()
            await iframe.screenshot(path=str(target))
            return {"path": str(target), "frame_selector": frame_selector}
        await (await self.current_page()).screenshot(path=str(target), full_page=full_page)
        return {"path": str(target)}

    async def image_bytes(self) -> bytes:
        return await (await self.current_page()).screenshot(type="png", full_page=False)

    async def viewport(self) -> dict:
        page = await self.current_page()
        size = page.viewport_size
        if size:
            return size
        return await page.evaluate(
            "() => ({ width: window.innerWidth, height: window.innerHeight })"
        )

    async def mouse_click(self, x: float, y: float) -> dict:
        await (await self.current_page()).mouse.click(x, y)
        return {"ok": True}

    async def keyboard_type(self, text: str) -> dict:
        await (await self.current_page()).keyboard.type(text)
        return {"ok": True}

    async def keyboard_press(self, key: str) -> dict:
        await (await self.current_page()).keyboard.press(key)
        return {"ok": True}

    async def reload(self) -> dict:
        await (await self.current_page()).reload(wait_until="domcontentloaded")
        return await self.snapshot()

    async def back(self) -> dict:
        page = await self.current_page()
        result = await page.go_back(wait_until="domcontentloaded")
        if result is None:
            return await self.snapshot()
        return await self.snapshot()

    async def wait(
        self,
        *,
        text: str | None = None,
        text_gone: str | None = None,
        url_contains: str | None = None,
        selector: str | None = None,
        frame_selector: str | None = None,
        state: str = "visible",
        timeout_ms: int = 5_000,
    ) -> dict:
        page = await self.current_page()
        scope = await self.scope(frame_selector)
        if url_contains:
            await page.wait_for_url(f"**{url_contains}**", timeout=timeout_ms)
        if selector:
            await scope.locator(selector).first.wait_for(timeout=timeout_ms, state=state)
        if text:
            await scope.get_by_text(text, exact=False).first.wait_for(
                timeout=timeout_ms,
                state=state,
            )
        if text_gone:
            await scope.get_by_text(text_gone, exact=False).first.wait_for(
                timeout=timeout_ms,
                state="hidden",
            )
        return await self.snapshot()

    async def list_tabs(self) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        tabs = []
        current = await self.current_page()
        for index, page in enumerate(self.context.pages):
            tabs.append(
                {
                    "index": index,
                    "url": page.url,
                    "title": await page.title(),
                    "active": page == current,
                }
            )
        return {"tabs": tabs}

    async def select_tab(self, index: int) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        pages = self.context.pages
        if index < 0 or index >= len(pages):
            raise RuntimeError(f"tab index out of range: {index}")
        self.page = pages[index]
        await self.page.bring_to_front()
        return await self.snapshot()

    async def new_tab(self, url: str | None = None) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        page = await self.context.new_page()
        self._attach_page(page)
        self.page = page
        if url:
            await page.goto(url, wait_until="domcontentloaded")
        return await self.snapshot()

    async def close_tab(self, index: int | None = None) -> dict:
        if not self.context:
            raise RuntimeError("browser not started")
        pages = self.context.pages
        if not pages:
            raise RuntimeError("no tabs to close")
        if index is None:
            page = await self.current_page()
        else:
            if index < 0 or index >= len(pages):
                raise RuntimeError(f"tab index out of range: {index}")
            page = pages[index]
        await page.close()
        remaining = self.context.pages
        self.page = remaining[0] if remaining else None
        if self.page:
            await self.page.bring_to_front()
            return await self.snapshot()
        return {"ok": True, "tabs": []}
