//! RLM background routing for parallel tool outputs.

use std::sync::Arc;

use crate::provider::Provider;
use crate::session::Session;
use crate::session::helper::rlm_background;

pub(super) fn route(
    session: &Session,
    model: &str,
    provider: Arc<dyn Provider>,
    out: &super::result::Output,
) -> String {
    let routed = rlm_background::route_or_defer(
        &out.content,
        &out.tool_name,
        &out.tool_input,
        &out.tool_id,
        &session.id,
        &session.messages,
        model,
        provider,
        &session.metadata.rlm,
        None,
    );
    crate::tool::feedback::render(&out.tool_name, out.success, &routed)
}
