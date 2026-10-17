use serde::{Deserialize, Serialize};

/// Telemetry and crash reporting settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_reporting: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_reporting_prompted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_report_endpoint: Option<String>,
}

impl TelemetryConfig {
    pub fn crash_reporting_enabled(&self) -> bool {
        self.crash_reporting.unwrap_or(false)
    }

    pub fn crash_reporting_prompted(&self) -> bool {
        self.crash_reporting_prompted.unwrap_or(false)
    }

    pub fn crash_report_endpoint(&self) -> String {
        self.crash_report_endpoint
            .clone()
            .unwrap_or_else(default_crash_report_endpoint)
    }
}

fn default_crash_report_endpoint() -> String {
    "https://api.codetether.run/v1/crash-reports".to_string()
}
