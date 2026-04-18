use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, request::EvalRequest};

pub(in crate::tool::browserctl) async fn eval(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = EvalRequest {
        source: require_string(&input.expression, "expression")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::execute(input, BrowserCommand::Eval(request)).await
}

pub(in crate::tool::browserctl) async fn console_eval(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = EvalRequest {
        source: require_string(&input.script, "script")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::execute(input, BrowserCommand::ConsoleEval(request)).await
}
