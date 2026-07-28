//! Persistent JavaScript runtime bridge for browser pages.

#[path = "runtime_init.rs"]
mod runtime_init;
#[path = "runtime_routes.rs"]
mod runtime_routes;

use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::PageLoadState;

impl BrowserPage {
    /// Execute inline scripts in the page's persistent JavaScript context.
    pub fn run_scripts(&mut self) -> Result<(), String> {
        self.enforce_resource_limits("page.run_scripts")?;
        self.sync_context_state_into_session();
        self.prepare_external_resources()?;
        self.install_agent_bridges()?;
        let checkpoint = self.event_checkpoint();
        let routes = self.active_route_handler();
        let result = {
            let runtime = self.runtime_mut();
            runtime.and_then(|runtime| {
                runtime.set_route_handler(routes);
                runtime.run_scripts()
            })
        };
        self.apply_runtime_result(checkpoint, "page.run_scripts", result)?;
        self.collect_agent_bridges()?;
        self.sync_context_state_from_session();
        self.mark_load_state(PageLoadState::NetworkIdle);
        Ok(())
    }

    /// Evaluate JavaScript in the page's persistent JavaScript context.
    pub fn eval_js(&mut self, script: &str) -> Result<crate::js::JsValue, String> {
        self.enforce_resource_limits("page.eval_js")?;
        self.sync_context_state_into_session();
        self.install_agent_bridges()?;
        let checkpoint = self.event_checkpoint();
        let routes = self.active_route_handler();
        let result = {
            let runtime = self.runtime_mut();
            runtime.and_then(|runtime| {
                runtime.set_route_handler(routes);
                runtime.eval(script)
            })
        };
        let value = self.apply_runtime_result(checkpoint, "page.eval_js", result)?;
        self.collect_agent_bridges()?;
        self.sync_context_state_from_session();
        self.mark_load_state(PageLoadState::NetworkIdle);
        Ok(value)
    }
}
