use crate::config::{Config, LspSettings, PermissionConfig, TelemetryConfig};

impl Config {
    pub(super) fn merge_permissions(&mut self, other: PermissionConfig) {
        self.permissions.rules.extend(other.rules);
        self.permissions.tools.extend(other.tools);
        self.permissions.paths.extend(other.paths);
    }

    pub(super) fn merge_telemetry(&mut self, other: TelemetryConfig) {
        if other.crash_reporting.is_some() {
            self.telemetry.crash_reporting = other.crash_reporting;
        }
        if other.crash_reporting_prompted.is_some() {
            self.telemetry.crash_reporting_prompted = other.crash_reporting_prompted;
        }
        if other.crash_report_endpoint.is_some() {
            self.telemetry.crash_report_endpoint = other.crash_report_endpoint;
        }
    }

    pub(super) fn merge_lsp(&mut self, other: LspSettings) {
        self.lsp.servers.extend(other.servers);
        self.lsp.linters.extend(other.linters);
        if other.disable_builtin_linters {
            self.lsp.disable_builtin_linters = true;
        }
    }
}
