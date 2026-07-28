//! Security checks applied before route matching.

use crate::browser_agent::exports::{RequestSecurityMetadata, SecurityPolicy};

use super::RouteRequest;

pub(crate) fn metadata_for(
    page_url: &str,
    request: &RouteRequest,
    policy: &SecurityPolicy,
) -> RequestSecurityMetadata {
    RequestSecurityMetadata::new(page_url, &request.url, policy)
}

pub(crate) fn blocked_reason(metadata: &RequestSecurityMetadata) -> Option<String> {
    if metadata.target_origin.is_opaque() || metadata.allowed_by_policy {
        return None;
    }
    Some(format!(
        "CORS blocked cross-origin request from {} to {}",
        metadata.request_origin.serialized(),
        metadata.target_origin.serialized()
    ))
}
