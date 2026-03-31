use super::super::Session;
use super::CodexImportReport;
use super::discover::find_codex_session_path_by_id;
use super::meta::read_session_meta;
use super::parse::parse_codex_session_from_path;
use super::paths::{codex_home_dir, codex_home_dir_for_path, native_sessions_dir};
use super::persist::persist_imported_session;
use anyhow::{Context, Result};

pub async fn import_codex_session_by_id(id: &str) -> Result<Session> {
    let Some(codex_home) = codex_home_dir() else {
        anyhow::bail!("Codex home directory not found");
    };
    let Some(path) = find_codex_session_path_by_id(&codex_home, id)? else {
        anyhow::bail!("Codex session not found: {id}");
    };
    import_codex_session_path(&path).await
}

pub async fn import_codex_session_path(path: &std::path::Path) -> Result<Session> {
    let Some(codex_home) = codex_home_dir_for_path(path) else {
        anyhow::bail!("Could not resolve Codex home for {}", path.display());
    };
    let meta = read_session_meta(path)?
        .with_context(|| format!("Missing session metadata in {}", path.display()))?;
    let session = parse_codex_session_from_path(
        path,
        super::index::load_session_index(&codex_home)
            .get(&meta.id)
            .map(String::as_str),
    )?;
    let _ = persist_imported_session(session, &native_sessions_dir()?).await?;
    Session::load(&meta.id).await
}

pub async fn import_codex_sessions_for_directory(
    dir: &std::path::Path,
) -> Result<CodexImportReport> {
    let mut report = CodexImportReport::default();
    for info in super::discover_codex_sessions_for_directory(dir)? {
        let session = parse_codex_session_from_path(&info.path, info.title.as_deref())?;
        match persist_imported_session(session, &native_sessions_dir()?).await? {
            super::info::PersistOutcome::Saved => report.imported += 1,
            super::info::PersistOutcome::Unchanged => report.skipped += 1,
        }
    }
    Ok(report)
}

pub async fn load_or_import_session(id: &str) -> Result<Session> {
    let existing = Session::load(id).await.ok();
    if let Some(codex_home) = codex_home_dir()
        && let Some(path) = find_codex_session_path_by_id(&codex_home, id)?
    {
        return import_codex_session_path(&path).await;
    }
    if let Some(existing) = existing {
        Ok(existing)
    } else {
        import_codex_session_by_id(id).await
    }
}
