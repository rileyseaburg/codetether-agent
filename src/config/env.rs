use crate::config::Config;
use crate::config::bool_parse::parse_bool;

impl Config {
    pub(super) fn apply_env(&mut self) {
        self.apply_default_env();
        self.apply_provider_env();
        self.apply_a2a_env();
        self.apply_telemetry_env();
    }

    fn apply_default_env(&mut self) {
        if let Ok(val) = std::env::var("CODETETHER_DEFAULT_MODEL") {
            self.default_model = Some(val);
        }
        if let Ok(val) = std::env::var("CODETETHER_DEFAULT_PROVIDER") {
            self.default_provider = Some(val);
        }
    }

    fn apply_provider_env(&mut self) {
        self.set_provider_key("OPENAI_API_KEY", "openai");
        self.set_provider_key("ANTHROPIC_API_KEY", "anthropic");
        self.set_provider_key("GOOGLE_API_KEY", "google");
    }

    fn set_provider_key(&mut self, env: &str, provider: &str) {
        if let Ok(val) = std::env::var(env) {
            self.providers
                .entry(provider.to_string())
                .or_default()
                .api_key = Some(val);
        }
    }
}

impl Config {
    fn apply_a2a_env(&mut self) {
        if let Ok(val) = std::env::var("CODETETHER_A2A_SERVER") {
            self.a2a.server_url = Some(val);
        }
    }

    fn apply_telemetry_env(&mut self) {
        if let Ok(val) = std::env::var("CODETETHER_CRASH_REPORTING") {
            match parse_bool(&val) {
                Ok(enabled) => self.telemetry.crash_reporting = Some(enabled),
                Err(_) => tracing::warn!(value = %val, "Invalid CODETETHER_CRASH_REPORTING"),
            }
        }
        if let Ok(val) = std::env::var("CODETETHER_CRASH_REPORT_ENDPOINT") {
            self.telemetry.crash_report_endpoint = Some(val);
        }
    }
}
