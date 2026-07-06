//! Actionable error mapping for Bedrock data-retention policy denials.
//!
//! Fable-class models declare `allowed_modes = ["provider_data_share"]`
//! in the Bedrock data-retention docs, so under the default/inherit mode
//! they resolve to `status: unavailable` and every invoke is blocked with
//! a 400 ("not available under data retention mode 'default'"). This is a
//! policy gate at invoke time, not a malformed request. We surface the
//! project-scoped opt-in fix instead of the bare HTTP error.
//! See anthropics/claude-code#66594.

/// Whether the error body is a data-retention policy denial.
pub(in crate::provider::bedrock) fn is_retention_denied(body: &str) -> bool {
    body.contains("data retention mode")
}

/// Wrap `base` with recovery guidance for a retention-mode denial.
pub(in crate::provider::bedrock) fn guidance(base: &str, model_id: &str, region: &str) -> String {
    format!(
        "{base}\n\
         `{model_id}` only permits the `provider_data_share` data-retention \
         mode, so it is policy-blocked under your effective mode \
         (project -> account -> model default).\n\
         Fix without touching other models: opt in a single Bedrock project \
         and route this model through it:\n\
         \x20\x20curl https://bedrock-mantle.{region}.api.aws/v1/organization/projects/<PROJECT_ID> \\\n\
         \x20\x20\x20\x20-H \"x-api-key: $BEDROCK_API_KEY\" -H \"Content-Type: application/json\" \\\n\
         \x20\x20\x20\x20-d '{{\"data_retention\":{{\"mode\":\"provider_data_share\"}}}}'\n\
         Verify the model reports `status: available`:\n\
         \x20\x20curl https://bedrock-mantle.{region}.api.aws/v1/models/anthropic.claude-fable-5 \\\n\
         \x20\x20\x20\x20-H \"x-api-key: $BEDROCK_API_KEY\"\n\
         Caveat: provider_data_share retains prompts/completions with \
         Anthropic (~30 days, safety review) outside the AWS boundary. If \
         you rely on Bedrock for ZDR/compliance, pin a non-Fable model \
         instead of opting in."
    )
}
