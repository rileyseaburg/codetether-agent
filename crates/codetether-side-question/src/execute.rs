//! Execute a resolved `/ask` request and render the answer.
//!
//! Isolated so [`crate::run_ask`] stays small enough to satisfy the
//! 50-line module budget while still carrying full rustdoc.

use std::sync::Arc;

use super::extract;
use crate::View;
use codetether_provider_core::{CompletionRequest, Provider};

/// Send `request` to `provider`, then render the reply or the error.
///
/// On success appends a system chat message of the
/// form `"/ask → <reply>"`; on failure only the view's status is
/// updated. Never mutates session history.
pub(super) async fn run<V: View>(
    view: &mut V,
    provider: Arc<dyn Provider>,
    request: CompletionRequest,
) -> bool {
    view.set_status("/ask: asking…".to_string());
    match provider.complete(request).await {
        Ok(resp) => {
            let text = extract::extract_text(&resp.message);
            view.push_system_message(format!("/ask → {text}"));
            view.set_status("/ask: answered".to_string());
            view.scroll_to_bottom();
            true
        }
        Err(err) => {
            view.set_status(format!("/ask failed: {err}"));
            false
        }
    }
}
