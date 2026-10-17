//! Crash reporting report.
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct CrashReport {
    pub(super) report_version: u32,
    pub(super) report_id: String,
    pub(super) occurred_at: DateTime<Utc>,
    pub(super) app_version: String,
    pub(super) command_line: String,
    pub(super) os: String,
    pub(super) arch: String,
    pub(super) process_id: u32,
    pub(super) thread_name: String,
    pub(super) panic_message: String,
    pub(super) panic_location: Option<String>,
    pub(super) backtrace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) memory: Option<codetether_telemetry::memory::MemorySnapshot>,
}

impl CrashReport {
    pub(super) fn from_panic_info(
        settings: &CrashReporterSettings,
        panic_info: &panic::PanicHookInfo<'_>,
    ) -> Self {
        let panic_message = panic_payload_to_string(panic_info);
        let panic_location = panic_info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));
        let thread_name = std::thread::current()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unnamed".to_string());
        let backtrace = Backtrace::force_capture().to_string();

        Self {
            report_version: REPORT_VERSION,
            report_id: Uuid::new_v4().to_string(),
            occurred_at: Utc::now(),
            app_version: settings.app_version.clone(),
            command_line: settings.command_line.clone(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            process_id: std::process::id(),
            thread_name,
            panic_message: truncate_with_ellipsis(&panic_message, MAX_PANIC_MESSAGE_CHARS),
            panic_location,
            backtrace: truncate_with_ellipsis(&backtrace, MAX_BACKTRACE_CHARS),
            memory: Some(codetether_telemetry::memory::MemorySnapshot::capture()),
        }
    }
}
