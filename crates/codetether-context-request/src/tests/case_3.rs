use super::fixtures::{request, text};
use crate::finish;
use codetether_provider_core::Role;

#[test]
fn actual_request_output_limit_is_reserved() {
    let mut request = request();
    request.max_tokens = Some(usize::MAX);
    assert!(finish(request, vec![]).is_err());
}
