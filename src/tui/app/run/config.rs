//! Native session and reviewer bindings for shared startup configuration.
use crate::{
    config::{AccessMode, Config},
    provider::ProviderRegistry,
    session::Session,
};
use std::sync::Arc;
struct Native<'a>(&'a mut Session);
impl codetether_tui_startup::configuration::Host for Native<'_> {
    type Registry = ProviderRegistry;
    fn set_trust(&mut self, config: &Config) {
        crate::tui::ui::trust_status::set_from_config(config);
    }
    fn install_review(&mut self, config: &Config, registry: &ProviderRegistry) {
        crate::review::runtime::install(config.review.clone(), Arc::new(registry.clone()));
    }
    fn apply_config(&mut self, config: &Config, registry: Option<&ProviderRegistry>) {
        self.0.apply_config(config, registry);
    }
    fn enable_auto_apply(&mut self) {
        self.0.metadata.auto_apply_edits = true;
    }
}
pub(super) fn apply_policies(
    session: &mut Session,
    startup: &mut super::startup::Startup,
    access_mode: Option<AccessMode>,
    yolo: bool,
) {
    codetether_tui_startup::configuration::apply_policies(
        &mut Native(session),
        startup,
        access_mode,
        yolo,
    );
}
