use super::index::load_session_index;
use super::info::CodexSessionInfo;
use super::meta::read_session_meta;
use super::parse::parse_codex_session_from_path;
use super::paths::{canonicalize_loose, codex_home_dir, is_jsonl};
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) fn discover_codex_sessions_for_directory(dir: &Path) -> Result<Vec<CodexSessionInfo>> {
    discover_codex_sessions_in(codex_home_dir().as_deref(), dir)
}

pub(crate) fn discover_codex_sessions_in(
    codex_home: Option<&Path>,
    dir: &Path,
) -> Result<Vec<CodexSessionInfo>> {
    let Some(codex_home) = codex_home else {
        return Ok(Vec::new());
    };
    let sessions_root = codex_home.join("sessions");
    if !sessions_root.exists() {
        return Ok(Vec::new());
    }

    let titles = load_session_index(codex_home);
    let canonical_dir = canonicalize_loose(dir);
    let mut sessions = Vec::new();

    for entry in WalkDir::new(&sessions_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        if !is_jsonl(path) {
            continue;
        }
        let meta = match read_session_meta(path) {
            Ok(Some(m)) => m,
            Ok(None) => continue,
            Err(err) => {
                tracing::warn!(path = %path.display(), error = %err, "Skipping unreadable Codex session meta");
                continue;
            }
        };
        if canonicalize_loose(Path::new(&meta.cwd)) != canonical_dir {
            continue;
        }
        match parse_codex_session_from_path(path, titles.get(&meta.id).map(String::as_str)) {
            Ok(session) => sessions.push(to_info(path.to_path_buf(), session)),
            Err(error) => {
                tracing::warn!(path = %path.display(), error = %error, "Skipping invalid Codex session")
            }
        }
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

pub(crate) fn find_codex_session_path_by_id(
    codex_home: &Path,
    id: &str,
) -> Result<Option<PathBuf>> {
    let sessions_root = codex_home.join("sessions");
    if !sessions_root.exists() {
        return Ok(None);
    }
    for entry in WalkDir::new(&sessions_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        if !is_jsonl(path) {
            continue;
        }
        if let Some(meta) = read_session_meta(path)?
            && meta.id == id
        {
            return Ok(Some(path.to_path_buf()));
        }
    }
    Ok(None)
}

fn to_info(path: PathBuf, session: super::super::Session) -> CodexSessionInfo {
    CodexSessionInfo {
        id: session.id,
        path,
        title: session.title,
        created_at: session.created_at,
        updated_at: session.updated_at,
        message_count: session.messages.len(),
        agent: session.agent,
        directory: session.metadata.directory,
    }
}
