//! JSON payload builder for allocator crash reports.

use chrono::Utc;
use serde_json::{Value, json};

use super::alloc_guard_command::command_line;
use super::memory::MemorySnapshot;

pub(crate) fn build(report_id: &str, size: usize, ceiling: usize, backtrace: &str) -> Value {
    json!({
        "report_version": 1,
        "report_id": report_id,
        "occurred_at": Utc::now().to_rfc3339(),
        "app_version": env!("CARGO_PKG_VERSION"),
        "command_line": command_line(),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "process_id": std::process::id(),
        "thread_name": std::thread::current().name().unwrap_or("unnamed"),
        "panic_message": format!(
            "alloc_guard: single allocation of {size} bytes exceeds ceiling {ceiling} bytes"
        ),
        "panic_location": null,
        "backtrace": backtrace,
        "memory": MemorySnapshot::capture(),
        "runtime_context": super::crash_context::snapshot(),
    })
}
