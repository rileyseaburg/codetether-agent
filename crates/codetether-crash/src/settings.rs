//! Crash reporting settings.
use super::*;
#[derive(Debug, Clone)]
pub(super) struct CrashReporterSettings {
    pub(super) enabled: bool,
    pub(super) endpoint: String,
    pub(super) report_dir: PathBuf,
    pub(super) app_version: String,
    pub(super) command_line: String,
    pub(super) install_id: String,
    pub(super) auth_token: Option<String>,
    pub(super) api_key: Option<String>,
}

impl CrashReporterSettings {
    pub(super) fn from_config(config: &Config, app_version: &str) -> Self {
        let install_id = match load_or_create_install_id() {
            Ok(id) => id,
            Err(err) => {
                tracing::warn!(error = %err, "Failed to load persistent install ID; using ephemeral value");
                Uuid::new_v4().to_string()
            }
        };

        Self {
            enabled: config.telemetry.crash_reporting_enabled(),
            endpoint: config.telemetry.crash_report_endpoint(),
            report_dir: crash_report_dir(),
            app_version: app_version.to_string(),
            command_line: truncate_with_ellipsis(
                &std::env::args().collect::<Vec<_>>().join(" "),
                MAX_COMMAND_CHARS,
            ),
            install_id,
            auth_token: env_non_empty(ENV_CRASH_REPORT_AUTH_TOKEN),
            api_key: env_non_empty(ENV_CRASH_REPORT_API_KEY),
        }
    }
}
