//! Bedrock service-tier cycling for the Settings panel.

use crate::provider::bedrock::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;
pub fn bedrock_service_tier_label() -> &'static str {
    codetether_settings_policy::bedrock_service_tier_label(
        runtime_config::service_tier().unwrap_or_default().as_str(),
    )
}

pub async fn cycle_bedrock_service_tier(app: &mut App, session: &mut Session) {
    let next = codetether_settings_policy::next_bedrock_service_tier(bedrock_service_tier_label());
    runtime_config::set_service_tier(next);
    persist(
        app,
        session,
        format!("Bedrock service tier: {}", bedrock_service_tier_label()),
    )
    .await;
}
