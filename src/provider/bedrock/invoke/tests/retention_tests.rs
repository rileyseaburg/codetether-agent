//! Tests for data-retention denial detection and guidance.

use crate::provider::bedrock::invoke::retention::{guidance, is_retention_denied};

const DENIAL: &str = "This model is not available under data retention mode 'default'.";

#[test]
fn detects_retention_denial_body() {
    assert!(is_retention_denied(DENIAL));
    assert!(!is_retention_denied("ThrottlingException: rate exceeded"));
    assert!(!is_retention_denied(
        "ValidationException: malformed request"
    ));
}

#[test]
fn guidance_names_project_scoped_fix_and_region() {
    let msg = guidance(
        &format!("Bedrock InvokeModel error (400 Bad Request): {DENIAL}"),
        "us.anthropic.claude-fable-5",
        "us-east-1",
    );
    assert!(msg.contains("400 Bad Request"));
    assert!(msg.contains("provider_data_share"));
    assert!(msg.contains("bedrock-mantle.us-east-1.api.aws/v1/organization/projects"));
    assert!(msg.contains("bedrock-mantle.us-east-1.api.aws/v1/models/anthropic.claude-fable-5"));
    assert!(msg.contains("status: available"));
    // ZDR/compliance caveat must be surfaced, not just the opt-in.
    assert!(msg.contains("ZDR"));
}
