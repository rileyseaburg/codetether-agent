//! In-memory implementation of RalphStateStore
//!
//! Used for tests and as a fallback when no external store is configured.

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use super::state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
use super::types::{Prd, ProgressEntry, RalphStatus};

/// In-memory state store backed by a RwLock<HashMap>
pub struct InMemoryStore {
    runs: RwLock<HashMap<String, RalphRunState>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            runs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RalphStateStore for InMemoryStore {
    async fn create_run(&self, state: &RalphRunState) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        runs.insert(state.run_id.clone(), state.clone());
        Ok(())
    }

    async fn update_status(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.status = status;
            if status == RalphStatus::Running && run.started_at.is_none() {
                run.started_at = Some(chrono::Utc::now().to_rfc3339());
            }
        }
        Ok(())
    }

    async fn update_iteration(&self, run_id: &str, iteration: usize) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.current_iteration = iteration;
        }
        Ok(())
    }

    async fn record_story_result(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            // Update or insert
            if let Some(existing) = run.story_results.iter_mut().find(|r| r.story_id == result.story_id) {
                *existing = result.clone();
            } else {
                run.story_results.push(result.clone());
            }
        }
        Ok(())
    }

    async fn append_progress(&self, run_id: &str, entry: &ProgressEntry) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.progress_log.push(entry.clone());
        }
        Ok(())
    }

    async fn update_prd(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.prd = prd.clone();
        }
        Ok(())
    }

    async fn set_error(&self, run_id: &str, error: &str) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.error = Some(error.to_string());
        }
        Ok(())
    }

    async fn complete_run(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            run.status = status;
            run.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
        Ok(())
    }

    async fn get_run(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>> {
        let runs = self.runs.read().await;
        Ok(runs.get(run_id).cloned())
    }

    async fn list_runs(&self) -> anyhow::Result<Vec<RalphRunSummary>> {
        let runs = self.runs.read().await;
        Ok(runs
            .values()
            .map(|run| RalphRunSummary {
                run_id: run.run_id.clone(),
                okr_id: run.okr_id.clone(),
                status: run.status,
                passed: run.prd.passed_count(),
                total: run.prd.user_stories.len(),
                current_iteration: run.current_iteration,
                created_at: run.created_at.clone(),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ralph::types::{Prd, QualityChecks, RalphConfig, UserStory};

    fn test_prd() -> Prd {
        Prd {
            project: "test".to_string(),
            feature: "test-feature".to_string(),
            branch_name: "feature/test".to_string(),
            version: "1.0".to_string(),
            user_stories: vec![UserStory {
                id: "US-001".to_string(),
                title: "Test story".to_string(),
                description: "A test story".to_string(),
                acceptance_criteria: vec![],
                verification_steps: vec![],
                passes: false,
                priority: 1,
                depends_on: vec![],
                complexity: 2,
            }],
            technical_requirements: vec![],
            quality_checks: QualityChecks::default(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    fn test_run_state() -> RalphRunState {
        RalphRunState {
            run_id: "test-run-1".to_string(),
            okr_id: Some("okr-1".to_string()),
            prd: test_prd(),
            config: RalphConfig::default(),
            status: RalphStatus::Pending,
            current_iteration: 0,
            max_iterations: 10,
            progress_log: vec![],
            story_results: vec![],
            error: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: None,
            completed_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_get_run() {
        let store = InMemoryStore::new();
        let state = test_run_state();

        store.create_run(&state).await.unwrap();

        let fetched = store.get_run("test-run-1").await.unwrap();
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.run_id, "test-run-1");
        assert_eq!(fetched.status, RalphStatus::Pending);
    }

    #[tokio::test]
    async fn test_update_status() {
        let store = InMemoryStore::new();
        store.create_run(&test_run_state()).await.unwrap();

        store
            .update_status("test-run-1", RalphStatus::Running)
            .await
            .unwrap();

        let run = store.get_run("test-run-1").await.unwrap().unwrap();
        assert_eq!(run.status, RalphStatus::Running);
        assert!(run.started_at.is_some());
    }

    #[tokio::test]
    async fn test_record_story_result() {
        let store = InMemoryStore::new();
        store.create_run(&test_run_state()).await.unwrap();

        let result = StoryResultEntry {
            story_id: "US-001".to_string(),
            title: "Test story".to_string(),
            passed: true,
            iteration: 1,
            error: None,
        };

        store
            .record_story_result("test-run-1", &result)
            .await
            .unwrap();

        let run = store.get_run("test-run-1").await.unwrap().unwrap();
        assert_eq!(run.story_results.len(), 1);
        assert!(run.story_results[0].passed);
    }

    #[tokio::test]
    async fn test_complete_run() {
        let store = InMemoryStore::new();
        store.create_run(&test_run_state()).await.unwrap();

        store
            .complete_run("test-run-1", RalphStatus::Completed)
            .await
            .unwrap();

        let run = store.get_run("test-run-1").await.unwrap().unwrap();
        assert_eq!(run.status, RalphStatus::Completed);
        assert!(run.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_list_runs() {
        let store = InMemoryStore::new();
        store.create_run(&test_run_state()).await.unwrap();

        let summaries = store.list_runs().await.unwrap();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].run_id, "test-run-1");
        assert_eq!(summaries[0].total, 1);
    }

    #[tokio::test]
    async fn test_get_nonexistent_run() {
        let store = InMemoryStore::new();
        let result = store.get_run("nonexistent").await.unwrap();
        assert!(result.is_none());
    }
}
