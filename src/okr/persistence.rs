//! OKR Persistence Layer
//!
//! Provides file-based persistence for OKR entities using Config::data_dir().
//! Supports CRUD operations for ID, run, and status queries.

use crate::okr::{Okr, OkrRun, OkrRunStatus, OkrStatus};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Repository for OKR persistence operations
pub struct OkrRepository {
    base_path: PathBuf,
    cache: Arc<RwLock<OkrCache>>,
}

/// In-memory cache for OKR data
#[derive(Debug, Default)]
struct OkrCache {
    okrs: Vec<Okr>,
    runs: Vec<OkrRun>,
}

impl OkrRepository {
    /// Create a new OKR repository with the given base path
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            cache: Arc::new(RwLock::new(OkrCache::default())),
        }
    }

    /// Create repository from Config::data_dir() or fallback to temp
    pub async fn from_config() -> Result<Self> {
        let base_path = crate::config::Config::data_dir()
            .map(|p| p.join("okr"))
            .unwrap_or_else(|| std::env::temp_dir().join("codetether").join("okr"));

        // Ensure directory exists
        fs::create_dir_all(&base_path)
            .await
            .context("Failed to create OKR data directory")?;

        tracing::info!(path = %base_path.display(), "OKR repository initialized");

        Ok(Self::new(base_path))
    }

    /// Get the file path for an OKR
    fn okr_path(&self, id: Uuid) -> PathBuf {
        self.base_path
            .join("objectives")
            .join(format!("{}.json", id))
    }

    /// Get the file path for an OKR run
    fn run_path(&self, id: Uuid) -> PathBuf {
        self.base_path.join("runs").join(format!("{}.json", id))
    }

    /// Get the directory path for OKRs
    fn okrs_dir(&self) -> PathBuf {
        self.base_path.join("objectives")
    }

    /// Get the directory path for runs
    fn runs_dir(&self) -> PathBuf {
        self.base_path.join("runs")
    }

    // ============ OKR CRUD ============

    /// Create a new OKR
    pub async fn create_okr(&self, okr: Okr) -> Result<Okr> {
        // Validate
        okr.validate()?;

        // Ensure directory exists
        fs::create_dir_all(self.okrs_dir()).await?;

        // Write to file (atomic: tmp + rename)
        let path = self.okr_path(okr.id);
        let tmp_path = path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(&okr)?;
        fs::write(&tmp_path, &json).await?;
        fs::rename(&tmp_path, &path).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.okrs.push(okr.clone());

        tracing::info!(okr_id = %okr.id, title = %okr.title, "OKR created");
        Ok(okr)
    }

    /// Get an OKR by ID
    pub async fn get_okr(&self, id: Uuid) -> Result<Option<Okr>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(okr) = cache.okrs.iter().find(|o| o.id == id) {
                return Ok(Some(okr.clone()));
            }
        }

        // Load from disk
        let path = self.okr_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await?;
        let okr: Okr = serde_json::from_str(&data).context("Failed to parse OKR")?;

        // Update cache
        let mut cache = self.cache.write().await;
        if !cache.okrs.iter().any(|o| o.id == okr.id) {
            cache.okrs.push(okr.clone());
        }

        Ok(Some(okr))
    }

    /// Update an existing OKR
    pub async fn update_okr(&self, mut okr: Okr) -> Result<Okr> {
        okr.updated_at = chrono::Utc::now();

        // Validate
        okr.validate()?;

        // Write to file
        let path = self.okr_path(okr.id);
        fs::create_dir_all(self.okrs_dir()).await?;
        let json = serde_json::to_string_pretty(&okr)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        if let Some(existing) = cache.okrs.iter_mut().find(|o| o.id == okr.id) {
            *existing = okr.clone();
        }

        tracing::info!(okr_id = %okr.id, "OKR updated");
        Ok(okr)
    }

    /// Delete an OKR by ID
    pub async fn delete_okr(&self, id: Uuid) -> Result<bool> {
        let path = self.okr_path(id);

        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).await?;

        // Remove from cache
        let mut cache = self.cache.write().await;
        cache.okrs.retain(|o| o.id != id);

        tracing::info!(okr_id = %id, "OKR deleted");
        Ok(true)
    }

    /// List all OKRs
    pub async fn list_okrs(&self) -> Result<Vec<Okr>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if !cache.okrs.is_empty() {
                return Ok(cache.okrs.clone());
            }
        }

        // Load from disk
        let dir = self.okrs_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut okrs = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json")
                && let Ok(data) = fs::read_to_string(&path).await
                && let Ok(okr) = serde_json::from_str::<Okr>(&data)
            {
                okrs.push(okr);
            }
        }

        // Update cache
        let mut cache = self.cache.write().await;
        cache.okrs = okrs.clone();

        Ok(okrs)
    }

    /// Query OKRs by status
    pub async fn query_okrs_by_status(&self, status: OkrStatus) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all.into_iter().filter(|o| o.status == status).collect())
    }

    /// Query OKRs by owner
    pub async fn query_okrs_by_owner(&self, owner: &str) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all
            .into_iter()
            .filter(|o| o.owner.as_deref() == Some(owner))
            .collect())
    }

    /// Query OKRs by tenant
    pub async fn query_okrs_by_tenant(&self, tenant_id: &str) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all
            .into_iter()
            .filter(|o| o.tenant_id.as_deref() == Some(tenant_id))
            .collect())
    }

    // ============ OKR Run CRUD ============

    /// Create a new OKR run
    pub async fn create_run(&self, run: OkrRun) -> Result<OkrRun> {
        // Validate
        run.validate()?;

        // Ensure directory exists
        fs::create_dir_all(self.runs_dir()).await?;

        // Write to file
        let path = self.run_path(run.id);
        let json = serde_json::to_string_pretty(&run)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.runs.push(run.clone());

        tracing::info!(run_id = %run.id, okr_id = %run.okr_id, name = %run.name, "OKR run created");
        Ok(run)
    }

    /// Get a run by ID
    pub async fn get_run(&self, id: Uuid) -> Result<Option<OkrRun>> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(run) = cache.runs.iter().find(|r| r.id == id) {
                return Ok(Some(run.clone()));
            }
        }

        // Load from disk
        let path = self.run_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await?;
        let run: OkrRun = serde_json::from_str(&data).context("Failed to parse OKR run")?;

        // Update cache
        let mut cache = self.cache.write().await;
        if !cache.runs.iter().any(|r| r.id == run.id) {
            cache.runs.push(run.clone());
        }

        Ok(Some(run))
    }

    /// Update a run
    pub async fn update_run(&self, mut run: OkrRun) -> Result<OkrRun> {
        run.updated_at = chrono::Utc::now();

        // Write to file
        let path = self.run_path(run.id);
        fs::create_dir_all(self.runs_dir()).await?;
        let json = serde_json::to_string_pretty(&run)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        if let Some(existing) = cache.runs.iter_mut().find(|r| r.id == run.id) {
            *existing = run.clone();
        }

        tracing::info!(run_id = %run.id, "OKR run updated");
        Ok(run)
    }

    /// Delete a run
    pub async fn delete_run(&self, id: Uuid) -> Result<bool> {
        let path = self.run_path(id);

        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).await?;

        // Remove from cache
        let mut cache = self.cache.write().await;
        cache.runs.retain(|r| r.id != id);

        tracing::info!(run_id = %id, "OKR run deleted");
        Ok(true)
    }

    /// List all runs
    pub async fn list_runs(&self) -> Result<Vec<OkrRun>> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if !cache.runs.is_empty() {
                return Ok(cache.runs.clone());
            }
        }

        // Load from disk
        let dir = self.runs_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut runs = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json")
                && let Ok(data) = fs::read_to_string(&path).await
                && let Ok(run) = serde_json::from_str::<OkrRun>(&data)
            {
                runs.push(run);
            }
        }

        // Update cache
        let mut cache = self.cache.write().await;
        cache.runs = runs.clone();

        Ok(runs)
    }

    /// Query runs by OKR ID
    pub async fn query_runs_by_okr(&self, okr_id: Uuid) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all.into_iter().filter(|r| r.okr_id == okr_id).collect())
    }

    /// Query runs by status
    pub async fn query_runs_by_status(&self, status: OkrRunStatus) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all.into_iter().filter(|r| r.status == status).collect())
    }

    /// Query runs by correlation ID
    pub async fn query_runs_by_correlation(&self, correlation_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.correlation_id.as_deref() == Some(correlation_id))
            .collect())
    }

    /// Query runs by relay checkpoint ID
    pub async fn query_runs_by_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.relay_checkpoint_id.as_deref() == Some(checkpoint_id))
            .collect())
    }

    /// Query runs by session ID
    pub async fn query_runs_by_session(&self, session_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.session_id.as_deref() == Some(session_id))
            .collect())
    }

    // ============ Utility Methods ============

    /// Clear the in-memory cache (useful for forcing reload from disk)
    #[allow(dead_code)]
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.okrs.clear();
        cache.runs.clear();
        tracing::debug!("OKR repository cache cleared");
    }

    /// Get statistics about the repository
    pub async fn stats(&self) -> Result<OkrRepositoryStats> {
        let okrs = self.list_okrs().await?;
        let runs = self.list_runs().await?;

        let okr_status_counts = okrs
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, o| {
                *acc.entry(o.status).or_insert(0) += 1;
                acc
            });

        let run_status_counts = runs
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, r| {
                *acc.entry(r.status).or_insert(0) += 1;
                acc
            });

        Ok(OkrRepositoryStats {
            total_okrs: okrs.len(),
            total_runs: runs.len(),
            okr_status_counts,
            run_status_counts,
        })
    }
}

/// Statistics about the OKR repository
#[derive(Debug, serde::Serialize)]
pub struct OkrRepositoryStats {
    pub total_okrs: usize,
    pub total_runs: usize,
    pub okr_status_counts: std::collections::HashMap<OkrStatus, usize>,
    pub run_status_counts: std::collections::HashMap<OkrRunStatus, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okr::KeyResult;

    fn temp_repo() -> OkrRepository {
        let temp_dir = std::env::temp_dir().join(format!("okr_test_{}", Uuid::new_v4()));
        OkrRepository::new(temp_dir)
    }

    #[tokio::test]
    async fn test_create_and_get_okr() {
        let repo = temp_repo();

        let mut okr = Okr::new("Test Objective", "Description");
        let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        okr.add_key_result(kr);

        let created = repo.create_okr(okr).await.unwrap();

        let fetched = repo.get_okr(created.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().title, "Test Objective");
    }

    #[tokio::test]
    async fn test_update_okr() {
        let repo = temp_repo();

        let mut okr = Okr::new("Test", "Desc");
        let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        okr.add_key_result(kr);

        let created = repo.create_okr(okr).await.unwrap();

        let mut to_update = created.clone();
        to_update.title = "Updated Title".to_string();
        to_update.status = OkrStatus::Active;

        let updated = repo.update_okr(to_update).await.unwrap();
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.status, OkrStatus::Active);
    }

    #[tokio::test]
    async fn test_delete_okr() {
        let repo = temp_repo();

        let mut okr = Okr::new("Test", "Desc");
        let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        okr.add_key_result(kr);

        let created = repo.create_okr(okr).await.unwrap();
        let deleted = repo.delete_okr(created.id).await.unwrap();
        assert!(deleted);

        let fetched = repo.get_okr(created.id).await.unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_query_okrs_by_status() {
        let repo = temp_repo();

        let mut okr1 = Okr::new("Active OKR", "Desc");
        let kr1 = KeyResult::new(okr1.id, "KR1", 100.0, "%");
        okr1.add_key_result(kr1);
        okr1.status = OkrStatus::Active;

        let mut okr2 = Okr::new("Draft OKR", "Desc");
        let kr2 = KeyResult::new(okr2.id, "KR2", 100.0, "%");
        okr2.add_key_result(kr2);
        okr2.status = OkrStatus::Draft;

        repo.create_okr(okr1).await.unwrap();
        repo.create_okr(okr2).await.unwrap();

        let active = repo.query_okrs_by_status(OkrStatus::Active).await.unwrap();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].title, "Active OKR");
    }

    #[tokio::test]
    async fn test_crud_runs() {
        let repo = temp_repo();

        // First create an OKR
        let mut okr = Okr::new("Test OKR", "Desc");
        let okr_id = okr.id;
        let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        okr.add_key_result(kr);
        repo.create_okr(okr).await.unwrap();

        // Create a run
        let mut run = OkrRun::new(okr_id, "Q1 Run");
        run.correlation_id = Some("corr-123".to_string());
        run.session_id = Some("session-456".to_string());

        let created = repo.create_run(run).await.unwrap();

        // Fetch by ID
        let fetched = repo.get_run(created.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "Q1 Run");

        // Query by OKR
        let runs_for_okr = repo.query_runs_by_okr(okr_id).await.unwrap();
        assert_eq!(runs_for_okr.len(), 1);

        // Query by correlation
        let runs_by_corr = repo.query_runs_by_correlation("corr-123").await.unwrap();
        assert_eq!(runs_by_corr.len(), 1);

        // Query by status
        let pending = repo
            .query_runs_by_status(OkrRunStatus::Draft)
            .await
            .unwrap();
        assert_eq!(pending.len(), 1);
    }

    #[tokio::test]
    async fn test_list_and_stats() {
        let repo = temp_repo();

        // Create OKRs
        let mut okr = Okr::new("Test", "Desc");
        let kr = KeyResult::new(okr.id, "KR", 100.0, "%");
        okr.add_key_result(kr);
        repo.create_okr(okr).await.unwrap();

        // List
        let all_okrs = repo.list_okrs().await.unwrap();
        assert_eq!(all_okrs.len(), 1);

        let all_runs = repo.list_runs().await.unwrap();
        assert_eq!(all_runs.len(), 0);

        // Stats
        let stats = repo.stats().await.unwrap();
        assert_eq!(stats.total_okrs, 1);
    }
}
