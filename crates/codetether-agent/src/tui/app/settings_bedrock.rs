//! Bedrock service-tier cycling for the Settings panel.

use crate::provider::bedrock::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

pub fn bedrock_service_tier_label() -> &'static str {
    codetether_settings_policy::bedrock_service_tier_label(
        runtime_config::service_tier().unwrap_or_default().as_str(),
    )
}

pub async fn cycle_bedrock_service_tier(app: &mut App, session: &mut Session) {
    super::host::cycle(
        app,
        session,
        codetether_settings_policy::cycles::Cycle::BedrockTier,
    )
    .await;
}
