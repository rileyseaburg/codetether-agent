use thiserror::Error;

#[path = "error_cdp.rs"]
mod cdp;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("browser command is not implemented: {0}")]
    NotImplemented(String),
    #[error("navigation timed out")]
    NavigationTimeout,
    #[error("element not found: {0}")]
    ElementNotFound(String),
    #[error("element is not fillable: tag={tag}, input_type={input_type:?}")]
    ElementNotFillable {
        tag: String,
        input_type: Option<String>,
    },
    #[error("element is not a file input: tag={tag}, input_type={input_type:?}")]
    ElementNotFileInput {
        tag: String,
        input_type: Option<String>,
    },
    #[error("file does not exist: {0}")]
    FileNotFound(String),
    #[error("file input does not accept multiple files: {0}")]
    MultipleFilesNotAllowed(String),
    #[error("waiting for selector timed out: {selector} after {timeout_ms}ms")]
    WaitTimeout { selector: String, timeout_ms: u64 },
    #[error("tab not found: {0}")]
    TabNotFound(usize),
    #[error("javascript exception: {message}{stack_suffix}", stack_suffix = stack_suffix(.stack.as_deref()))]
    JsException {
        message: String,
        stack: Option<String>,
    },
    #[error("browser session not started")]
    SessionNotStarted,
    #[error("tab is closed")]
    TabClosed,
    #[error("browser crashed or connection was lost")]
    BrowserCrashed,
    #[error("evaluation timed out")]
    EvaluationTimeout,
    #[error("evaluation promise rejected: {message}")]
    EvalRejection { message: String },
    #[error(
        "evaluation returned non-serializable value: {description} (type={type_}, subtype={subtype:?})"
    )]
    EvalNonSerializable {
        type_: String,
        subtype: Option<String>,
        description: String,
    },
    #[error("evaluation wrapper error: {0}")]
    EvalWrapperError(String),
    #[error("browser operation failed: {0}")]
    OperationFailed(String),
}

impl From<anyhow::Error> for BrowserError {
    fn from(error: anyhow::Error) -> Self {
        Self::OperationFailed(error.to_string())
    }
}

impl From<serde_json::Error> for BrowserError {
    fn from(error: serde_json::Error) -> Self {
        Self::OperationFailed(error.to_string())
    }
}

fn stack_suffix(stack: Option<&str>) -> String {
    stack.map(|value| format!("\n{value}")).unwrap_or_default()
}
