//! Typed tool-result emission with the legacy text fallback.
use super::*;
pub(super) fn success(
    tool_name: &str,
    result: &ToolResult,
    started: Instant,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    // Emit the typed event when a worker sink is installed; otherwise
    // fall back to the historical text-only callback. Flattening the
    // tool name, status, and output into one string is what previously
    // made every transcript entry publish as `agent.message`.
    let emitted = crate::events::sink::emit_tool_result_if_active(
        tool_name,
        result.success,
        &result.output,
        Some(started.elapsed().as_millis()),
    );
    if !emitted && let Some(cb) = cb {
        cb(format!(
            "[tool:{}:{}] {}",
            tool_name,
            if result.success { "ok" } else { "err" },
            codetether_util::truncate::truncate_bytes_safe(&result.output, 500)
        ));
    }
}
