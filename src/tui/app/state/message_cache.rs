//! Shared rendered-line cache methods with native regression tests.
codetether_message_cache::impl_app_cache!(super::AppState);
#[cfg(test)]
#[path = "message_cache_tests.rs"]
mod tests;
