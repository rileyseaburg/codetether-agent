use crate::config::bool_parse::parse_bool;
use crate::config::{Config, PermissionProfileConfig};
use anyhow::Result;

impl Config {
    pub(super) fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default_provider" => self.default_provider = Some(value.to_string()),
            "default_model" => self.default_model = Some(value.to_string()),
            "access_mode" => self.access_mode = Some(super::set_parse::parse_string(value)?),
            "sandbox_mode" => self.sandbox_mode = Some(super::set_parse::parse_string(value)?),
            "approval_policy" => {
                self.approval_policy = Some(super::set_parse::parse_string(value)?)
            }
            "trust_level" => self.trust_level = Some(super::set_parse::parse_string(value)?),
            "permission_profile" => {
                self.permission_profile = Some(PermissionProfileConfig::Named(
                    super::set_parse::parse_string(value)?,
                ))
            }
            "a2a.server_url" => self.a2a.server_url = Some(value.to_string()),
            "a2a.worker_name" => self.a2a.worker_name = Some(value.to_string()),
            "ui.theme" => self.ui.theme = value.to_string(),
            "telemetry.crash_reporting" => {
                self.telemetry.crash_reporting = Some(parse_bool(value)?)
            }
            "telemetry.crash_reporting_prompted" => {
                self.telemetry.crash_reporting_prompted = Some(parse_bool(value)?)
            }
            "telemetry.crash_report_endpoint" => {
                self.telemetry.crash_report_endpoint = Some(value.to_string())
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        Ok(())
    }
}
