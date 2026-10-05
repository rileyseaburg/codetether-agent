//! On-disk persistence: save, load, delete, and directory lookup.

#[cfg(test)]
use std::path::{Path, PathBuf};

use anyhow::Result;

#[cfg(test)]
use super::header::SessionHeader;
use super::tail_load::TailLoad;
#[cfg(test)]
use super::tail_seed::with_tail_cap;
use super::types::Session;

mod canonical_root;
mod location;
mod paths;
#[cfg(test)]
#[path = "save_guard.rs"]
mod save_guard;
#[cfg(test)]
#[path = "persistence_snapshot.rs"]
mod snapshot;
#[path = "workspace_resolve.rs"]
mod workspace_resolve;

impl Session {
    /// Load an existing session by its UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the session file does not exist, exceeds the
    /// sanity size cap, or the JSON is malformed.
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let mut session = super::store::load(&path, usize::MAX).await?.session;
        anyhow::ensure!(
            session.id == id,
            "Session file identity does not match requested ID"
        );
        session.normalize_sidecars();
        Ok(session)
    }

    /// Load the most recent session, optionally scoped to a workspace
    /// directory.
    ///
    /// When `workspace` is [`Some`], only considers sessions created in that
    /// directory. When [`None`], returns the most recent session globally.
    ///
    /// # Errors
    ///
    /// Returns an error if no sessions exist (or, with `workspace` set,
    /// none match the requested directory).
    pub async fn last_for_directory(workspace: Option<&std::path::Path>) -> Result<Self> {
        Self::last_for_directory_tail(workspace, usize::MAX)
            .await
            .map(|t| t.session)
    }

    /// Like [`Self::last_for_directory`] but keeps only the last `window`
    /// messages and tool uses in memory, returning a [`TailLoad`] with the
    /// number of entries that were dropped. Use this when resuming very
    /// large sessions where the full transcript would exhaust memory.
    ///
    /// Listing combines database headers with unmigrated legacy entries.
    /// The selected session is resumed through the indexed store rather than
    /// parsing the transcript to discard its prefix.
    pub async fn last_for_directory_tail(
        workspace: Option<&std::path::Path>,
        window: usize,
    ) -> Result<TailLoad> {
        let sessions_dir = Self::sessions_dir()?;
        let canonical_workspace = workspace.map(|w| {
            w.canonicalize().unwrap_or_else(|e| {
                tracing::warn!(path = %w.display(), error = %e, "canonicalize failed; using raw path");
                w.to_path_buf()
            })
        });
        let sessions = match canonical_workspace {
            Some(workspace) => super::listing::list_sessions_for_directory(&workspace).await?,
            None => super::list_sessions().await?,
        };
        let latest = sessions
            .first()
            .ok_or_else(|| anyhow::anyhow!("No sessions found"))?;
        Self::load_tail(&latest.id, window).await
    }

    /// Load the most recent session globally (unscoped).
    ///
    /// Kept for legacy compatibility; prefer
    /// [`Session::last_for_directory`].
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Atomically flush changed records and metadata to the indexed session store.
    ///
    /// Unchanged sessions do not write to disk or enqueue background work. Changed
    /// sessions encode only dirty suffixes; the transcript prefix is never cloned,
    /// hashed, or rewritten. SQLite transactions run on a blocking worker.
    ///
    /// # Errors
    /// Returns storage errors or `SESSION_REVISION_CONFLICT` for a stale writer.
    /// Reload a conflicting session instead of silently replacing newer records.
    /// A legacy JSON file must be imported through the load API before saving.
    ///
    /// For a full JSON snapshot use [`Self::export_json`] explicitly. Per-session
    /// `.json` files are now small locators, not the canonical transcript.
    ///
    pub async fn save(&self) -> Result<()> {
        let path = self
            .storage
            .path_for(&self.id)
            .map(Ok)
            .unwrap_or_else(|| Self::session_path(&self.id))?;
        if super::store::save(self, &path).await? {
            location::record(&self.id, &path);
            super::index::recall::schedule(self);
            super::index_produce::notify::saved(self);
            super::store::archive::schedule(self);
        }
        Ok(())
    }

    /// Delete a session file by ID. No-op if the file does not exist.
    pub async fn delete(id: &str) -> Result<()> {
        let path = Self::session_path(id)?;
        super::index_produce::notify::removed(id).await;
        super::index::recall::remove(id).await?;
        super::store::delete::run(&path, id).await?;
        match tokio::fs::remove_file(&path).await {
            Ok(()) => {
                location::forget(id);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                location::forget(id);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}

/// Index-first scan. Tries the O(1) workspace index; on miss or stale
/// entry, falls back to the byte-prefiltered directory scan and repairs
/// the index with the winner so the next launch is O(1).
#[cfg(test)]
fn scan_with_index(
    sessions_dir: &Path,
    canonical_workspace: Option<PathBuf>,
    window: usize,
) -> Result<TailLoad> {
    // Fast path: check the sidecar index.
    if let Some(ws) = canonical_workspace.as_ref() {
        let index = super::workspace_index::WorkspaceIndex::load_sync();
        if let Some(id) = index.get(ws) {
            let candidate = sessions_dir.join(format!("{id}.json"));
            if candidate.exists() {
                if let Ok(load) = tail_load_sync(&candidate, window) {
                    // Confirm it still belongs to this workspace — the user
                    // could have edited the session's metadata.directory
                    // manually or moved a file.
                    let dir_ok = load
                        .session
                        .metadata
                        .directory
                        .as_ref()
                        .map(|d| {
                            let canonical = d.canonicalize().unwrap_or_else(|_| d.clone());
                            &canonical == ws
                        })
                        .unwrap_or(false);
                    if dir_ok {
                        return Ok(load);
                    }
                    tracing::warn!(
                        session_id = %id,
                        stored_dir = ?load.session.metadata.directory,
                        expected_dir = ?ws,
                        "Index hit but directory mismatch; falling back to scan"
                    );
                } else {
                    tracing::warn!(
                        session_id = %id,
                        path = %candidate.display(),
                        "Index hit but session file failed to parse; falling back to scan"
                    );
                }
            }
        }
    }

    // Slow path: scan everything.
    let result = scan_sync(sessions_dir, canonical_workspace.clone(), window);

    // Repair the index with whatever we found, so next time is O(1).
    if let (Ok(load), Some(ws)) = (&result, canonical_workspace.as_ref()) {
        let _ = super::workspace_index_io::upsert_sync(ws, &load.session.id);
    }

    result
}

/// Tail-load a specific path synchronously (used by the index fast path).
#[cfg(test)]
fn tail_load_sync(path: &Path, window: usize) -> Result<TailLoad> {
    use std::fs;
    use std::io::BufReader;
    let file_bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let file = fs::File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    let (parsed, dropped) = with_tail_cap(window, || serde_json::from_reader::<_, Session>(reader));
    let mut session = parsed?;
    session.normalize_sidecars();
    Ok(TailLoad {
        session,
        dropped,
        file_bytes,
    })
}

/// Fully synchronous directory scan. Lives inside `spawn_blocking`.
///
/// Flow:
/// 1. `read_dir` + stat every entry once to build `(path, mtime)` pairs.
/// 2. Sort newest-first.
/// 3. For each candidate, do a header-only parse (`SessionHeader`) — no
///    `Vec<Message>` allocation. Because `metadata` is serialized before
///    `messages`/`tool_uses` in new files, this is O(header bytes); older
///    files still work but pay a lex-through cost.
/// 4. On workspace match, re-open and do one tail-capped full parse.
#[cfg(test)]
fn scan_sync(
    sessions_dir: &Path,
    canonical_workspace: Option<PathBuf>,
    window: usize,
) -> Result<TailLoad> {
    use std::fs;
    use std::io::BufReader;
    use std::time::SystemTime;

    if !sessions_dir.exists() {
        anyhow::bail!("No sessions found");
    }

    let mut candidates: Vec<(PathBuf, SystemTime)> = Vec::new();
    for entry in fs::read_dir(sessions_dir)? {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                tracing::warn!(error = %err, "skipping unreadable directory entry");
                continue;
            }
        };
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let mtime = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        candidates.push((path, mtime));
    }
    if candidates.is_empty() {
        anyhow::bail!("No sessions found");
    }
    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    // Precompute a cheap byte-level needle for the workspace path. JSON
    // serializes `metadata.directory` as `"directory":"<path>"`, so if
    // the raw file bytes don't contain the path as a substring, we can
    // skip it without invoking serde_json at all.
    //
    // This is the single biggest win for large workspaces: a 10 MB
    // session that is *not* for this cwd gets ruled out in a few ms of
    // byte scanning instead of a full JSON lex.
    let needle: Option<Vec<u8>> = canonical_workspace.as_ref().map(|ws| {
        // JSON-escape the path (backslashes on Windows become `\\`,
        // quotes become `\"`). `serde_json::to_string` handles all the
        // edge cases for us.
        let quoted = serde_json::to_string(&ws.to_string_lossy()).unwrap_or_default();
        // Strip the surrounding quotes; we want the inner bytes so we
        // match whether the JSON has `"directory":"..."` or any other
        // surrounding context.
        let inner = quoted
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(&quoted);
        inner.as_bytes().to_vec()
    });
    // Build the SIMD-accelerated substring finder once for the whole
    // scan loop; reusing it across files is much faster than rebuilding.
    let finder = needle
        .as_ref()
        .map(|n| memchr::memmem::Finder::new(n.as_slice()).into_owned());

    // Parallel byte-prefilter: for each candidate, compute whether the
    // workspace path bytes appear in the file. This is the expensive
    // O(file_bytes) step. Running it concurrently across CPU cores
    // turns "sum of all file scan times" into ~"longest single file
    // scan time" on a machine with multiple cores.
    //
    // We preserve mtime ordering by collecting results into a parallel
    // Vec<bool> indexed the same as `candidates`, then iterating
    // serially to find the first hit.
    let prefilter_hits: Vec<bool> = match (finder.as_ref(), candidates.len()) {
        (None, _) => vec![true; candidates.len()],
        (Some(_), 0..=1) => vec![true; candidates.len()], // not worth spawning
        (Some(finder), _) => {
            let paths: Vec<&Path> = candidates.iter().map(|(p, _)| p.as_path()).collect();
            let results: std::sync::Mutex<Vec<Option<bool>>> =
                std::sync::Mutex::new(vec![None; paths.len()]);
            std::thread::scope(|scope| {
                // Chunk candidates across available CPUs. For ~300 files
                // a fan-out of 4-8 threads saturates I/O and CPU nicely
                // without oversubscribing.
                let threads = std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4)
                    .min(8);
                let chunk_size = paths.len().div_ceil(threads);
                for chunk_idx in 0..threads {
                    let start = chunk_idx * chunk_size;
                    if start >= paths.len() {
                        break;
                    }
                    let end = (start + chunk_size).min(paths.len());
                    let chunk_paths = &paths[start..end];
                    let results = &results;
                    scope.spawn(move || {
                        for (offset, p) in chunk_paths.iter().enumerate() {
                            let hit = file_contains_finder(p, finder).unwrap_or(false);
                            // Lock-per-entry is fine: contention is
                            // negligible vs. the ~ms file scan cost.
                            if let Ok(mut guard) = results.lock() {
                                guard[start + offset] = Some(hit);
                            }
                        }
                    });
                }
            });
            results
                .into_inner()
                .unwrap_or_default()
                .into_iter()
                .map(|o| o.unwrap_or(false))
                .collect()
        }
    };

    for (idx, (path, _)) in candidates.iter().enumerate() {
        // Fast path: byte-level substring prefilter (precomputed in parallel).
        if !prefilter_hits.get(idx).copied().unwrap_or(false) {
            continue;
        }

        // Slower path: full JSON header parse to confirm the match
        // (the substring test has false positives: the path could
        // appear in a chat message or tool output).
        let header_ok = (|| -> Result<bool> {
            let file = fs::File::open(path)?;
            let reader = BufReader::with_capacity(16 * 1024, file);
            let header: SessionHeader = match serde_json::from_reader(reader) {
                Ok(h) => h,
                Err(_) => return Ok(false),
            };
            if let Some(ref ws) = canonical_workspace {
                let Some(dir) = header.metadata.directory.as_ref() else {
                    return Ok(false);
                };
                if dir == ws {
                    return Ok(true);
                }
                let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
                Ok(&canonical_dir == ws)
            } else {
                Ok(true)
            }
        })();

        match header_ok {
            Ok(true) => {}
            Ok(false) => continue,
            Err(err) => {
                tracing::warn!(
                    path = %path.display(),
                    error = %err,
                    "skipping unreadable session file",
                );
                continue;
            }
        }

        // Match found — do the tail-capped full parse.
        let file_bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        let file = fs::File::open(path)?;
        let reader = BufReader::with_capacity(64 * 1024, file);
        let (parsed, dropped) =
            with_tail_cap(window, || serde_json::from_reader::<_, Session>(reader));
        return Ok(TailLoad {
            session: parsed?,
            dropped,
            file_bytes,
        });
    }

    anyhow::bail!("No sessions found")
}

/// Memory-map `path` and return `true` if the SIMD finder matches.
///
/// mmap avoids the per-chunk `read` syscall overhead and the
/// carry-over bookkeeping of a streaming scan: `memmem::find` runs
/// straight over the OS-provided virtual memory window, and the kernel
/// pages in only what's actually touched. On a ~10 MB session file
/// this is measurably faster than chunked `BufRead` + SIMD.
///
/// Falls back to returning `false` on any I/O error — the caller's
/// mtime loop will simply try the next candidate.
#[cfg(test)]
fn file_contains_finder(path: &Path, finder: &memchr::memmem::Finder<'_>) -> Result<bool> {
    use std::fs;

    let needle_len = finder.needle().len();
    if needle_len == 0 {
        return Ok(true);
    }
    let file = fs::File::open(path)?;
    let meta = file.metadata()?;
    let len = meta.len();
    if (len as usize) < needle_len {
        return Ok(false);
    }
    const MAX_MMAP_SIZE: u64 = 64 * 1024 * 1024; // 64 MB
    if len > MAX_MMAP_SIZE {
        tracing::warn!(path = %path.display(), size = len, "Skipping oversized session file");
        return Ok(false);
    }
    // SAFETY: We only read the mapping, we do not mutate it. The
    // kernel COW-protects the mapping and the `Mmap` owns the lifetime
    // tied to `file`, which we keep alive for the duration of `find`.
    // Concurrent external modification of the file could theoretically
    // tear the mapping, but session files are only ever rewritten
    // wholesale via atomic rename — never in-place truncated.
    let mmap = unsafe { memmap2::Mmap::map(&file)? };
    Ok(finder.find(&mmap[..]).is_some())
}
