//! ESLint JSON output types.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct FileReport {
    #[serde(rename = "filePath")]
    file_path: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    #[serde(rename = "ruleId")]
    rule_id: Option<String>,
    severity: u8,
    message: String,
    line: u32,
    column: u32,
}

impl FileReport {
    pub(super) fn render(self) -> Vec<String> {
        let path = self.file_path;
        self.messages
            .into_iter()
            .map(|message| message.render(&path))
            .collect()
    }
}

impl Message {
    fn render(self, path: &str) -> String {
        let severity = match self.severity {
            2 => "error",
            1 => "warning",
            _ => "info",
        };
        let code = self
            .rule_id
            .map(|rule_id| format!(" ({rule_id})"))
            .unwrap_or_default();
        format!(
            "[{severity}] {path}:{}:{} [eslint-cli]{} {}",
            self.line,
            self.column,
            code,
            self.message.replace('\n', " ")
        )
    }
}
