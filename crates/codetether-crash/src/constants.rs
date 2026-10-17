//! Crash report schema, limits, and credential environment names.

pub(super) const REPORT_VERSION: u32 = 1;

pub(super) const CRASH_SCHEMA: &str = "codetether.crash.v1";

pub(super) const CRASH_SOURCE: &str = "codetether-agent";

pub(super) const MAX_PENDING_REPORTS: usize = 50;

pub(super) const MAX_PANIC_MESSAGE_CHARS: usize = 2048;

pub(super) const MAX_BACKTRACE_CHARS: usize = 32_000;

pub(super) const MAX_COMMAND_CHARS: usize = 1024;

pub(super) const CRASH_UPLOAD_STATE_FILE: &str = "crash-upload-state.json";

pub(super) const MIN_UPLOAD_BACKOFF_SECS: u64 = 5 * 60;
// 5 minutes
pub(super) const MAX_UPLOAD_BACKOFF_SECS: u64 = 24 * 60 * 60;
// 24 hours
pub(super) const MAX_STATUS_FAILURES_PER_FLUSH: usize = 3;

pub(super) const ENV_CRASH_REPORT_AUTH_TOKEN: &str = "CODETETHER_CRASH_REPORT_AUTH_TOKEN";

pub(super) const ENV_CRASH_REPORT_API_KEY: &str = "CODETETHER_CRASH_REPORT_API_KEY";
