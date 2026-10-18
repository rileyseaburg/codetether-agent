//! Shared viewport paging with native regression tests.
codetether_chat_layout::impl_app_chat_paging!(super::AppState);
#[cfg(test)]
#[path = "scroll_page_tests.rs"]
mod tests;
