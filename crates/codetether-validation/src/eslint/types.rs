//! ESLint JSON output types.

use super::message::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct FileReport {
    #[serde(rename = "filePath")]
    file_path: String,
    messages: Vec<Message>,
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
