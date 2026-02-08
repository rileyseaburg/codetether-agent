//! Crash reporting (opt-in)
//!
//! Captures panic metadata into local spool files and, when enabled, ships
//! pending reports to a remote ingestion endpoint on next startup.

use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;
use uuid::Uuid;

const REPORT_VERSION: u32 = 1;
const MAX_PENDING_REPORTS: usize = 50;
const MAX_PANIC_MESSAGE_CHARS: usize = 2048;
const MAX_BACKTRACE_CHARS: usize = 32_000;
const MAX_COMMAND_CHARS: usize = 1024;

#[derive(Debug, Clone)]
struct CrashReporterSettings {
    enabled: bool,
    endpoint: String,
    report_dir: PathBuf,
    app_version: String,
    command_line: String,
}

impl CrashReporterSettings {
    fn from_config(config: &Config) -> Self {
        Self {
            enabled: config.telemetry.crash_reporting_enabled(),
            endpoint: config.telemetry.crash_report_endpoint(),
            report_dir: crash_report_dir(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            command_line: truncate_with_ellipsis(
                &std::env::args().collect::<Vec<_>>().join(" "),
                MAX_COMMAND_CHARS,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrashReport {
    report_version: u32,
    report_id: String,
    occurred_at: DateTime<Utc>,
    app_version: String,
    command_line: String,
    os: String,
    arch: String,
    process_id: u32,
    thread_name: String,
    panic_message: String,
    panic_location: Option<String>,
    backtrace: String,
}

impl CrashReport {
    fn from_panic_info(
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
        }
    }
}

pub async fn initialize(config: &Config) {
    let settings = CrashReporterSettings::from_config(config);
    install_panic_hook(settings.clone());

    if !settings.enabled {
        return;
    }

    if let Err(err) = flush_pending_reports(&settings).await {
        tracing::warn!(error = %err, "Failed to flush pending crash reports");
    }
}

fn install_panic_hook(settings: CrashReporterSettings) {
    static PANIC_HOOK_ONCE: Once = Once::new();
    PANIC_HOOK_ONCE.call_once(|| {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let persisted = persist_crash_report(&settings, panic_info);
            default_hook(panic_info);

            match persisted {
                Ok(path) if settings.enabled => {
                    eprintln!(
                        "codetether: crash report queued at '{}' and will be sent on next startup.",
                        path.display()
                    );
                }
                Ok(path) => {
                    eprintln!(
                        "codetether: crash report saved at '{}'. Crash reporting is disabled (opt-in).",
                        path.display()
                    );
                    eprintln!(
                        "codetether: enable with `codetether config --set telemetry.crash_reporting=true`"
                    );
                }
                Err(err) => {
                    eprintln!("codetether: failed to persist crash report: {err}");
                }
            }
        }));
    });
}

fn persist_crash_report(
    settings: &CrashReporterSettings,
    panic_info: &panic::PanicHookInfo<'_>,
) -> Result<PathBuf> {
    std::fs::create_dir_all(&settings.report_dir)
        .with_context(|| format!("create report dir {}", settings.report_dir.display()))?;

    let report = CrashReport::from_panic_info(settings, panic_info);
    let file_name = format!(
        "{}-{}.json",
        report.occurred_at.format("%Y%m%dT%H%M%S%.3fZ"),
        report.report_id
    );
    let path = settings.report_dir.join(file_name);
    let payload = serde_json::to_string_pretty(&report)?;
    std::fs::write(&path, payload).with_context(|| format!("write report {}", path.display()))?;

    prune_old_reports(&settings.report_dir, MAX_PENDING_REPORTS)?;
    Ok(path)
}

fn prune_old_reports(report_dir: &Path, max_reports: usize) -> Result<()> {
    let mut reports = pending_report_paths(report_dir)?;
    if reports.len() <= max_reports {
        return Ok(());
    }

    reports.sort_by_key(|path| {
        std::fs::metadata(path)
            .ok()
            .and_then(|meta| meta.modified().ok())
    });

    let remove_count = reports.len().saturating_sub(max_reports);
    for path in reports.into_iter().take(remove_count) {
        if let Err(err) = std::fs::remove_file(&path) {
            tracing::warn!(path = %path.display(), error = %err, "Failed pruning old crash report");
        }
    }

    Ok(())
}

fn pending_report_paths(report_dir: &Path) -> Result<Vec<PathBuf>> {
    if !report_dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    for entry in std::fs::read_dir(report_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            paths.push(path);
        }
    }

    paths.sort_by_key(|path| {
        std::fs::metadata(path)
            .ok()
            .and_then(|meta| meta.modified().ok())
    });

    Ok(paths)
}

async fn flush_pending_reports(settings: &CrashReporterSettings) -> Result<()> {
    let paths = pending_report_paths(&settings.report_dir)?;
    if paths.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent(format!("codetether/{}", settings.app_version))
        .build()
        .context("build crash reporting HTTP client")?;

    let mut sent = 0usize;
    let mut failed = 0usize;

    for path in paths {
        let raw = match tokio::fs::read_to_string(&path).await {
            Ok(content) => content,
            Err(err) => {
                failed += 1;
                tracing::warn!(path = %path.display(), error = %err, "Failed reading crash report");
                continue;
            }
        };

        let report: CrashReport = match serde_json::from_str(&raw) {
            Ok(report) => report,
            Err(err) => {
                failed += 1;
                tracing::warn!(
                    path = %path.display(),
                    error = %err,
                    "Invalid crash report format; dropping file"
                );
                let _ = tokio::fs::remove_file(&path).await;
                continue;
            }
        };

        let response = client.post(&settings.endpoint).json(&report).send().await;
        match response {
            Ok(resp) if resp.status().is_success() => {
                sent += 1;
                if let Err(err) = tokio::fs::remove_file(&path).await {
                    tracing::warn!(
                        path = %path.display(),
                        error = %err,
                        "Failed deleting uploaded crash report"
                    );
                }
            }
            Ok(resp) => {
                failed += 1;
                tracing::warn!(
                    path = %path.display(),
                    status = %resp.status(),
                    "Crash report upload failed"
                );
            }
            Err(err) => {
                failed += 1;
                tracing::warn!(
                    path = %path.display(),
                    error = %err,
                    "Crash report upload request failed"
                );
            }
        }
    }

    tracing::info!(
        sent = sent,
        failed = failed,
        endpoint = %settings.endpoint,
        "Crash report sync complete"
    );

    Ok(())
}

fn crash_report_dir() -> PathBuf {
    Config::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp/codetether-agent"))
        .join("crash-reports")
}

fn panic_payload_to_string(panic_info: &panic::PanicHookInfo<'_>) -> String {
    if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
        (*msg).to_string()
    } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
        msg.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}
