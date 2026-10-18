//! Bind history page loading to the native session store.
use super::types::{PageResult, Request};
pub(super) async fn run(request: Request) -> PageResult {
    codetether_history_page::load::<super::source::NativeSource>(request).await
}
