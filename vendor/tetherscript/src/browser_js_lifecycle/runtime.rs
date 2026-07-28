//! Runtime-facing navigation lifecycle dispatch methods.

use super::super::*;

impl BrowserJsRuntime {
    pub(crate) fn dispatch_beforeunload(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_plain_with_this("beforeunload", self.runtime.window.clone())?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn dispatch_unload(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_plain_with_this("unload", self.runtime.window.clone())?;
        self.settle(JsValue::Undefined)
    }

    #[allow(dead_code)]
    pub(crate) fn dispatch_pagehide(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_plain_with_this("pagehide", self.runtime.window.clone())?;
        self.settle(JsValue::Undefined)
    }

    #[allow(dead_code)]
    pub(crate) fn dispatch_pageshow(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_plain_with_this("pageshow", self.runtime.window.clone())?;
        self.settle(JsValue::Undefined)
    }

    #[allow(dead_code)]
    pub(crate) fn dispatch_visibilitychange(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_plain_with_this("visibilitychange", self.runtime.window.clone())?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn dispatch_hashchange(
        &mut self,
        old_url: &str,
        new_url: &str,
    ) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_hashchange_with_this(
            self.runtime.window.clone(),
            old_url,
            new_url,
        )?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn dispatch_popstate(&mut self) -> Result<BrowserJsResult, String> {
        super::dispatch::dispatch_popstate_with_this(self.runtime.window.clone(), JsValue::Null)?;
        self.settle(JsValue::Undefined)
    }
}
