//! Browserctl JavaScript evaluation action adapter.

use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, request::EvalRequest};

/// Build and execute a JavaScript evaluation command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `expression` is missing or execution fails.
pub(in crate::tool::browserctl) async fn eval(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    frame_scope(input)?;
    let request = EvalRequest {
        expression: require_string(&input.expression, "expression")?.to_string(),
        timeout_ms: input.timeout_ms.unwrap_or(30_000),
    };
    super::execute(input, BrowserCommand::Eval(request)).await
}

fn frame_scope(input: &BrowserCtlInput) -> Result<(), crate::browser::BrowserError> {
    if input
        .frame_selector
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return Err(crate::browser::BrowserError::OperationFailed(
            "frame-scoped eval is not implemented yet".into(),
        ));
    }
    Ok(())
}
