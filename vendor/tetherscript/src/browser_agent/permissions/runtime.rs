//! Runtime bridge installation for permission emulation.

use super::script;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::security::Origin;
use crate::js::JsValue;

impl BrowserPage {
    pub(crate) fn install_agent_bridges(&mut self) -> Result<(), String> {
        self.install_service_worker_js_bridge()?;
        self.sync_runtime_html();
        self.install_permissions_bridge()?;
        self.sync_runtime_html();
        self.install_dialog_bridge()?;
        self.sync_runtime_html();
        Ok(())
    }

    pub(crate) fn collect_agent_bridges(&mut self) -> Result<(), String> {
        self.collect_service_worker_js_bridge()?;
        self.collect_permissions_bridge()?;
        self.collect_dialogs()
    }

    pub(crate) fn install_permissions_bridge(&mut self) -> Result<(), String> {
        let origin = Origin::parse(&self.session.url);
        let states = self.permissions.bridge_states(&origin);
        let script = script::install(&states, &self.geolocation, &self.read_clipboard());
        self.runtime_mut()?.eval(&script)?;
        Ok(())
    }

    pub(crate) fn collect_permissions_bridge(&mut self) -> Result<(), String> {
        let value = self.runtime_mut()?.eval(script::drain())?.value;
        if let JsValue::String(text) = value {
            self.write_clipboard(text);
        }
        Ok(())
    }

    fn sync_runtime_html(&mut self) {
        if let Some(runtime) = self.runtime.as_mut() {
            self.session.html = runtime.html();
        }
    }
}
