//! JavaScript snippets for deterministic file-input metadata.

use crate::browser_agent::keyboard_escape::{node, quote};
use crate::browser_agent::FilePayload;

pub(crate) fn set_files(path: &[usize], files: &[FilePayload]) -> String {
    let metadata = metadata_json(files);
    let value = files
        .first()
        .map(|file| format!("C:\\fakepath\\{}", file.name))
        .unwrap_or_default();
    format!(
        "let n={}; n.value={}; n.setAttribute('data-agent-file-count',{}); \
         n.setAttribute('data-agent-files',{}); n.dispatchEvent('input'); \
         n.dispatchEvent('change');",
        node(path),
        quote(&value),
        quote(&files.len().to_string()),
        quote(&metadata)
    )
}

fn metadata_json(files: &[FilePayload]) -> String {
    let items = files
        .iter()
        .map(|file| {
            format!(
                "{{\"name\":\"{}\",\"type\":\"{}\",\"size\":{}}}",
                json(&file.name),
                json(&file.mime_type),
                file.byte_len()
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", items.join(","))
}

fn json(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            ch => vec![ch],
        })
        .collect()
}
