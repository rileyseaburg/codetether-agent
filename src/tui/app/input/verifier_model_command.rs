//! Native entry point for the shared verifier-model command.
use crate::{provider::ProviderRegistry, tui::app::state::App};
use std::sync::Arc;
pub(super) fn run(app: &mut App, registry: Option<&Arc<ProviderRegistry>>, prompt: &str) -> bool {
    crate::tui::app::model_picker::verifier_command(app, registry, prompt)
}
