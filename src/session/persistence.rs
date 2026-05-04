//! On-disk persistence: save, load, delete, and directory lookup.

use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;

use super::header::SessionHeader;
use super::tail_load::TailLoad;
use super::tail_seed::with_tail_cap;
use super::types::Session;

impl Session {
    /// Load an existing session by its UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the session file does not exist or the JSON is
    /// malformed.
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let content = fs::read_to_string(&path).await?;
        let mut session: Session = serde_json::from_str(&content)?;
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
    /// Implementation: the entire scan runs on a single blocking thread.
    /// For each candidate (newest-mtime first) we do a cheap
    /// [`SessionHeader`] parse to compare `metadata.directory`; only the
    /// matching file pays for a full tail parse.
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
        tokio::task::spawn_blocking(move || {
            scan_with_index(&sessions_dir, canonical_workspace, window)
        })
        .await
        .map_err(|e| anyhow::anyhow!("session scan task panicked: {e}"))?
    }

    /// Load the most recent session globally (unscoped).
    ///
    /// Kept for legacy compatibility; prefer
    /// [`Session::last_for_directory`].
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Persist the session to disk as JSON. Creates the sessions directory
    /// on demand.
    ///
    /// Performance:
    /// - Serialization runs on the blocking thread pool so a large session
    ///   doesn't stall the async reactor during `serde_json` formatting.
    /// - Writes **compact** JSON (no pretty-printing): ~30% less CPU to
    ///   serialize, ~30–40% smaller on disk, and correspondingly faster
    ///   to load and mmap-scan on resume. Session files are machine-owned
    ///   — humans never hand-edit them — so indentation is pure overhead.
    /// - Write is atomic (tmp + rename) so a crash mid-save leaves the
    ///   previous session intact, and the mmap prefilter in
    ///   [`file_contains_finder`] cannot observe a torn buffer.
    pub async fn save(&self) -> Result<()> {
        let path = Self::session_path(&self.id)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let tmp = path.with_extension("json.tmp");
        // Clone-and-serialize off the reactor. The clone is a Vec-copy
        // (~memcpy speed) which is always cheaper than the JSON
        // formatting we are about to avoid blocking the reactor on.
        let mut snapshot = self.clone();
        snapshot.normalize_sidecars();
        let sink_config = snapshot.metadata.history_sink.clone().or_else(|| {
            super::history_sink::HistorySinkConfig::from_env()
                .ok()
                .flatten()
        });
        let session_id_for_journal = snapshot.id.clone();
        let content = tokio::task::spawn_blocking(move || serde_json::to_vec(&snapshot))
            .await
            .map_err(|e| anyhow::anyhow!("session serialize task panicked: {e}"))??;
        fs::write(&tmp, content).await?;
        // Atomic swap. On POSIX `rename` is atomic over existing files;
        // on Windows we fall back to remove-then-rename.
        if let Err(primary) = fs::rename(&tmp, &path).await {
            let _ = fs::remove_file(&path).await;
            if let Err(retry) = fs::rename(&tmp, &path).await {
                let _ = fs::remove_file(&tmp).await;
                return Err(anyhow::anyhow!(
                    "session rename failed: {primary} (retry: {retry})"
                ));
            }
        }
        let mut journal = super::journal::WritebackJournal::new(&session_id_for_journal);
        let tx = journal.stage(super::journal::Op::Save);
        if let Err(reason) = journal.commit(tx) {
            journal.reject(tx, reason);
        }
        if let Err(err) =
            super::journal::append_entries(&session_id_for_journal, journal.entries()).await
        {
            tracing::warn!(
                %err,
                session_id = %session_id_for_journal,
                "save journal append failed (non-fatal)"
            );
        }
        // Update the workspace index so the next resume is O(1). Best
        // effort — a failed index write must not fail the session save.
        if let Some(dir) = &self.metadata.directory {
            let canonical = dir.canonicalize().unwrap_or_else(|_| dir.clone());
            let session_id = self.id.clone();
            tokio::task::spawn_blocking(move || {
                if let Err(err) = super::workspace_index_io::upsert_sync(&canonical, &session_id) {
                    tracing::debug!(%err, "workspace index upsert failed (non-fatal)");
                }
            });
        }
        // Phase A history sink: stream pure history to MinIO/S3.
        // Env-gated, fire-and-forget — never blocks the save, never
        // fails it. See [`super::history_sink`] for the env variables.
        //
        // Coalesce concurrent saves per session: if an upload for this
        // session is already in flight, skip this spawn — the next
        // save will pick up the latest history. This prevents bursty
        // save loops (e.g. during long tool chains) from queueing
        // unbounded background uploads.
        if let Some(sink_config) = sink_config {
            use std::collections::HashSet;
            use std::sync::{Mutex, OnceLock};
            static IN_FLIGHT: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
            let in_flight = IN_FLIGHT.get_or_init(|| Mutex::new(HashSet::new()));

            let inserted = in_flight
                .lock()
                .map(|mut s| s.insert(self.id.clone()))
                .unwrap_or(false);

            if inserted {
                let session_id = self.id.clone();
                let messages = self.messages.clone();
                tokio::spawn(async move {
                    if let Err(err) = super::history_sink::upload_full_history(
                        &sink_config,
                        &session_id,
                        &messages,
                    )
                    .await
                    {
                        tracing::warn!(%err, %session_id, "history sink upload failed (non-fatal)");
                    }
                    if let Ok(mut s) = in_flight.lock() {
                        s.remove(&session_id);
                    }
                });
            } else {
                tracing::debug!(
                    session_id = %self.id,
                    "history sink upload already in flight; skipping duplicate"
                );
            }
        }
        Ok(())
    }

    /// Delete a session file by ID. No-op if the file does not exist.
    pub async fn delete(id: &str) -> Result<()> {
        let path = Self::session_path(id)?;
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Resolve the sessions directory (`<data_dir>/sessions`).
    ///
    /// Cached after first resolution: `Config::data_dir()` does env-var
    /// lookups and filesystem checks which are cheap individually but add
    /// up across hundreds of save/load calls over a session's lifetime.
    pub(crate) fn sessions_dir() -> Result<PathBuf> {
        use std::sync::OnceLock;
        static CACHED: OnceLock<PathBuf> = OnceLock::new();
        if let Some(dir) = CACHED.get() {
            return Ok(dir.clone());
        }
        let dir = crate::config::Config::data_dir()
            .map(|d| d.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
        // Best-effort set; if another thread won the race we just use
        // theirs.
        let _ = CACHED.set(dir.clone());
        Ok(dir)
    }

    /// Resolve the on-disk path for a session file.
    pub(crate) fn session_path(id: &str) -> Result<PathBuf> {
        if id.is_empty() || id.len() > 128 || id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
            anyhow::bail!("Invalid session ID:rejecting path traversal risk");
        }
        Ok(Self::sessions_dir()?.join(format!("{}.json", id)))
    }
}

/// Index-first scan. Tries the O(1) workspace index; on miss or stale
/// entry, falls back to the byte-prefiltered directory scan and repairs
/// the index with the winner so the next launch is O(1).
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
