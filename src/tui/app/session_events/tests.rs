#[path = "tests_done.rs"]
mod tests_done;
#[path = "tests_retry.rs"]
mod tests_retry;
#[path = "tests_text.rs"]
mod tests_text;
#[path = "tests_tool.rs"]
mod tests_tool;
#[path = "tests_tool_status.rs"]
mod tests_tool_status;
#[path = "tests_usage.rs"]
mod tests_usage;

async fn test_slot() -> crate::tui::app::session_runtime::SessionSlot {
    let session = crate::session::Session::new().await.expect("session");
    crate::tui::app::session_runtime::SessionSlot::new(session)
}
