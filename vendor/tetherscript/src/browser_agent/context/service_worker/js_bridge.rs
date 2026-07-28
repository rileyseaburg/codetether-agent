//! JavaScript bridge installation for service workers and CacheStorage.

#[path = "js_bridge/apply.rs"]
mod apply;
#[path = "js_bridge/install.rs"]
mod install;
#[path = "js_bridge/operations.rs"]
mod operations;
#[path = "js_bridge/script.rs"]
mod script;
#[path = "js_bridge/script_data.rs"]
mod script_data;

use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub(crate) fn install_service_worker_js_bridge(&mut self) -> Result<(), String> {
        let script = install::script(self);
        self.runtime_mut()?.eval(&script)?;
        Ok(())
    }

    pub(crate) fn collect_service_worker_js_bridge(&mut self) -> Result<(), String> {
        let value = self.runtime_mut()?.eval(script::drain())?.value;
        let ops = operations::parse(&value);
        apply::ops(self, ops)
    }
}
