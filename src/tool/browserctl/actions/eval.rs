use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, request::EvalRequest};

pub(in crate::tool::browserctl) async fn eval(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    frame_scope(input)?;
    let request = EvalRequest {
        expression: require_string(&input.expression, "expression")?.to_string(),
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
