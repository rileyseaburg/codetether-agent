impl OpenAiCodexProvider {
    fn apply_service_tier(payload: &mut Value, service_tier: Option<CodexServiceTier>) {
        if let Some(service_tier) = service_tier {
            payload["service_tier"] = json!(service_tier.as_str());
        }
    }
}
