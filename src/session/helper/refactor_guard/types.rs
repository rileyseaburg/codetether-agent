use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GuardReport {
    pub files: Vec<GuardFile>,
    pub violations: Vec<GuardViolation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GuardFile {
    pub path: String,
    pub status: FileStatus,
    pub old_code_lines: Option<usize>,
    pub new_code_lines: usize,
    pub limit: usize,
    pub wrapper_target: Option<String>,
    #[serde(skip_serializing)]
    pub old_text: Option<String>,
    #[serde(skip_serializing)]
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Added,
    Modified,
}

#[derive(Debug, Clone, Serialize)]
pub struct GuardViolation {
    pub path: String,
    pub message: String,
}

impl GuardReport {
    pub fn new(files: Vec<GuardFile>) -> Self {
        Self {
            files,
            violations: Vec::new(),
        }
    }
}

impl GuardViolation {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}
