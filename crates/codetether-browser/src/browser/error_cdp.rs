use super::BrowserError;
use chromiumoxide::cdp::js_protocol::runtime::{ExceptionDetails, StackTrace};
use chromiumoxide::error::CdpError;

impl From<CdpError> for BrowserError {
    fn from(error: CdpError) -> Self {
        match error {
            CdpError::FrameNotFound(_) | CdpError::NotFound => Self::ElementNotFound("page".into()),
            CdpError::JavascriptException(details) => exception(*details),
            CdpError::NoResponse | CdpError::Ws(_) | CdpError::ChannelSendError(_) => {
                Self::BrowserCrashed
            }
            CdpError::Timeout => Self::NavigationTimeout,
            other => Self::OperationFailed(other.to_string()),
        }
    }
}

fn exception(details: ExceptionDetails) -> BrowserError {
    let message = details
        .exception
        .as_ref()
        .and_then(|value| value.description.as_deref().map(summary))
        .unwrap_or_else(|| details.text.clone());
    if details.text.contains("in promise") {
        return BrowserError::EvalRejection { message };
    }
    BrowserError::JsException {
        message,
        stack: details.stack_trace.map(stack),
    }
}

fn stack(trace: StackTrace) -> String {
    trace
        .call_frames
        .into_iter()
        .map(|frame| {
            format!(
                "at {} ({}:{}:{})",
                frame.function_name,
                frame.url,
                frame.line_number + 1,
                frame.column_number + 1
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn summary(value: &str) -> String {
    value.lines().next().unwrap_or(value).to_string()
}
