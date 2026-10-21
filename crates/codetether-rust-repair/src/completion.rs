//! One bounded provider call without tool, agent, or session capabilities.
use crate::{Proposal, RepairRequest, parse_proposal, validate_request};
use anyhow::{Context, Result, ensure};
use codetether_provider_core::{
    CompletionRequest, ContentPart, FinishReason, Message, Provider, Role,
};

/// Build the single tool-free request used by this backend.
/// # Arguments
/// `model` is the resolved model; `snapshot` is the editor's bounded context.
/// # Returns
/// A two-message request with no tools, no temperature override and 8,000 output tokens.
/// # Errors
/// Rejects invalid snapshot contracts before provider invocation.
/// # Examples
/// ```
/// let r = codetether_rust_repair::RepairRequest { schema_version:1, diagnostics:vec![], files:vec![] };
/// let c = codetether_rust_repair::completion_request("model".into(), &r).unwrap();
/// assert!(c.tools.is_empty());
/// ```
pub fn completion_request(model: String, snapshot: &RepairRequest) -> Result<CompletionRequest> {
    validate_request(snapshot)?;
    Ok(CompletionRequest {
        model,
        messages: vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: crate::prompt::SYSTEM.into(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: serde_json::to_string(snapshot)?,
                }],
            },
        ],
        tools: vec![],
        temperature: None,
        top_p: None,
        max_tokens: Some(8000),
        stop: vec![],
    })
}

/// Ask a provider exactly once and validate its text-only proposal.
/// # Arguments
/// `provider` is an existing native provider; `model` is resolved; `snapshot` is immutable.
/// # Returns
/// A validated proposal; this function never executes model tools or edits files.
/// # Errors
/// Propagates provider errors and rejects timeouts, truncation, tools/nontext or invalid JSON.
/// # Examples
/// ```no_run
/// # async fn example(p: &dyn codetether_provider_core::Provider, r: &codetether_rust_repair::RepairRequest) -> anyhow::Result<()> {
/// let _proposal = codetether_rust_repair::propose(p, "model".into(), r).await?;
/// # Ok(()) }
/// ```
pub async fn propose(
    provider: &dyn Provider,
    model: String,
    snapshot: &RepairRequest,
) -> Result<Proposal> {
    let request = completion_request(model.clone(), snapshot)?;
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(120),
        provider.complete(request),
    )
    .await
    .context("repair provider timed out after 120 seconds")?
    .with_context(|| {
        format!(
            "repair completion failed: provider={}, resolved_model={model}",
            provider.name()
        )
    })?;
    ensure!(
        response.finish_reason == FinishReason::Stop,
        "repair completion did not finish normally: {:?}",
        response.finish_reason
    );
    ensure!(
        response.message.role == Role::Assistant,
        "repair response must be an assistant message"
    );
    let mut text = String::new();
    for part in response.message.content {
        let ContentPart::Text { text: value } = part else {
            anyhow::bail!("repair response contains tool calls or nontext content")
        };
        text.push_str(&value);
    }
    parse_proposal(&text, snapshot)
}
