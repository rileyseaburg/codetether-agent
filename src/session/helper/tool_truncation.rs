use crate::provider::{ContentPart, Message, Role};

pub(in crate::session::helper) fn error_content(tool: &str) -> String {
    let retry_rule = if tool == "batch" {
        "Do not call `batch` on the retry; issue at most three individual tool calls with short arguments."
    } else {
        "Retry with one smaller tool call; split writes, shell scripts, or long arguments into separate turns."
    };
    format!(
        "Error: Your tool call to `{tool}` was truncated because the provider cut off the \
         arguments JSON before it could be parsed. Recovery required: {retry_rule} Do not \
         provide a final answer until the smaller tool call succeeds or you have tried a \
         different small diagnostic step."
    )
}

pub(in crate::session::helper) fn retry_prompt(tools: &[(String, String)]) -> Message {
    let names = tools
        .iter()
        .map(|(_, name)| name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let batch_rule = if tools.iter().any(|(_, name)| name == "batch") {
        " Do not use `batch` for this retry."
    } else {
        ""
    };
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "Runtime recovery directive: the previous tool call JSON was truncated ({names}). \
                 Retry now with smaller individual tool calls, at most three in this step.{batch_rule} \
                 Do not summarize or stop before attempting the smaller call."
            ),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::{error_content, retry_prompt};

    #[test]
    fn batch_recovery_forbids_batch_retry() {
        let content = error_content("batch");
        let text = format!("{:?}", retry_prompt(&[("call-1".into(), "batch".into())]));
        assert!(content.contains("Do not call `batch`"));
        assert!(content.contains("at most three individual tool calls"));
        assert!(text.contains("Do not use `batch`"));
    }
}
