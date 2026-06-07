use super::info::CodexSessionInfo;
use super::meta::read_session_meta;
use super::summary_scan::scan_codex_summary;
use super::title::derive_title_from_text;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(crate) fn summarize_codex_session_from_path(
    path: &Path,
    title_override: Option<&str>,
) -> Result<CodexSessionInfo> {
    let meta = read_session_meta(path)?
        .with_context(|| format!("Missing session_meta in {}", path.display()))?;
    let scan = scan_codex_summary(path, meta.timestamp)?;
    let id = meta.id;
    Ok(CodexSessionInfo {
        id: id.clone(),
        path: path.to_path_buf(),
        title: title_override.map(str::to_string).or_else(|| {
            scan.first_user_text
                .and_then(|text| derive_title_from_text(&text))
        }),
        created_at: meta.timestamp,
        updated_at: scan.updated_at.unwrap_or(meta.timestamp),
        message_count: scan.message_count,
        agent: "build".to_string(),
        directory: directory_from_cwd(&meta.cwd),
    })
}

fn directory_from_cwd(cwd: &str) -> Option<PathBuf> {
    (!cwd.is_empty()).then(|| {
        let raw = PathBuf::from(cwd);
        match raw.canonicalize() {
            Ok(canon) => canon,
            Err(_) => {
                tracing::warn!(path = %cwd, "Codex import: cwd does not resolve, storing as-is");
                raw
            }
        }
    })
}
