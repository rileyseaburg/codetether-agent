//! Ralph loop - the core autonomous execution loop

use super::state_store::{RalphRunState, RalphStateStore, StoryResultEntry};
use super::types::*;
use super::verification::run_story_verification;
use super::{git_policy, quality_shell};
use crate::bus::AgentBus;
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::provenance::{
    ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance_blocking,
};
use crate::provider::{ContentPart, Message, Provider, ProviderRegistry, Role};
use crate::session::delegation::DelegationState;
use crate::session::{Session, SessionEvent};
use crate::swarm::{executor::AgentLoopExit, run_agent_loop};
use crate::tool::ToolRegistry;
use crate::tui::ralph_view::{RalphEvent, RalphStoryInfo, RalphStoryStatus};
use crate::tui::swarm_view::SwarmEvent;
use crate::worktree::WorktreeManager;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

mod bus_collect;
mod bus_publish;
mod configure;
mod conflict_execute;
mod conflict_result;
mod conflicts;
mod construct;
mod event_bridge;
mod events;
mod git_checkout;
mod git_commit;
mod git_commit_dir;
mod lifecycle_cleanup;
mod lifecycle_finish;
mod lifecycle_record;
mod lifecycle_start;
mod persistence;
mod progress;
mod prompts;
mod provenance;
mod quality;
mod relay_announce;
mod relay_coordinator;
mod relay_events;
mod relay_finish;
mod relay_prepare;
mod relay_progress;
mod relay_rounds;
mod relay_sender;
mod relay_session;
mod relay_text;
mod relay_turn;
mod relay_types;
mod seq_error;
mod seq_failure;
mod seq_iteration;
mod seq_prompt;
mod seq_response;
mod seq_success;
mod seq_unchecked;
mod sequential;
mod status;

/// The main Ralph executor
pub struct RalphLoop {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
    event_tx: Option<mpsc::Sender<RalphEvent>>,
    bus: Option<Arc<AgentBus>>,
    registry: Option<Arc<ProviderRegistry>>,
    store: Option<Arc<dyn RalphStateStore>>,
    /// Optional CADMAS-CTX delegation state for LCB relay provider selection
    delegation: Option<DelegationState>,
    run_id: String,
}

impl RalphLoop {
    /// Run the Ralph loop until completion or max iterations
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        self.start_run()?;
        // Choose execution mode
        if self.config.parallel_enabled {
            self.run_parallel().await?;
        } else {
            self.run_sequential().await?;
        }

        codetether_ralph_core::lifecycle::finalize_status(&mut self.state);
        self.cleanup_run_worktrees().await;
        self.finish_run()
    }

    /// Run stories in parallel by stage with worktree isolation
    async fn run_parallel(&mut self) -> anyhow::Result<()> {
        // Clone stages upfront to avoid borrow issues
        let stages: Vec<Vec<UserStory>> = self
            .state
            .prd
            .stages()
            .into_iter()
            .map(|stage| stage.into_iter().cloned().collect())
            .collect();
        let total_stages = stages.len();

        info!(
            "Parallel execution: {} stages, {} max concurrent stories",
            total_stages, self.config.max_concurrent_stories
        );

        // Create worktree manager if enabled
        let worktree_mgr = if self.config.worktree_enabled {
            let mgr = WorktreeManager::for_repo(&self.state.working_dir).without_vscode_auto_open();
            info!("Worktree isolation enabled for parallel stories");
            Some(Arc::new(mgr))
        } else {
            None
        };

        for (stage_idx, stage_stories) in stages.into_iter().enumerate() {
            if self.state.prd.is_complete() {
                info!("All stories complete!");
                self.state.status = RalphStatus::Completed;
                break;
            }

            if self.state.current_iteration >= self.state.max_iterations {
                break;
            }

            let story_count = stage_stories.len();
            info!(
                "=== Stage {}/{}: {} stories in parallel ===",
                stage_idx + 1,
                total_stages,
                story_count
            );

            // Stories are already cloned from stages()
            let stories: Vec<UserStory> = stage_stories;

            // Pre-create all worktrees for this stage in parallel *before*
            // spawning the story tasks. `git worktree add` serializes on the
            // `.git/worktrees` lock, so doing this up front (bounded to 8)
            // drains the lock contention in one burst rather than racing it
            // against every story's first tool call.
            let mut worktrees_by_story: HashMap<String, crate::worktree::WorktreeInfo> =
                HashMap::new();
            if let Some(ref mgr) = worktree_mgr {
                let ids: Vec<String> = stories.iter().map(|s| s.id.clone()).collect();
                let created: Vec<(String, Option<crate::worktree::WorktreeInfo>)> =
                    stream::iter(ids.into_iter())
                        .map(|story_id| {
                            let mgr = Arc::clone(mgr);
                            async move {
                                let slug = story_id.to_lowercase().replace("-", "_");
                                match mgr.create(&slug).await {
                                    Ok(wt) => {
                                        if let Err(e) = mgr.inject_workspace_stub(&wt.path) {
                                            warn!(
                                                story_id = %story_id,
                                                error = %e,
                                                "Failed to inject workspace stub"
                                            );
                                        }
                                        info!(
                                            story_id = %story_id,
                                            worktree_path = %wt.path.display(),
                                            "Pre-created worktree for story"
                                        );
                                        (story_id, Some(wt))
                                    }
                                    Err(e) => {
                                        warn!(
                                            story_id = %story_id,
                                            error = %e,
                                            "Failed to pre-create worktree, will use main directory"
                                        );
                                        (story_id, None)
                                    }
                                }
                            }
                        })
                        .buffer_unordered(8)
                        .collect()
                        .await;
                for (sid, wt) in created {
                    if let Some(wt) = wt {
                        worktrees_by_story.insert(sid, wt);
                    }
                }
            }

            // Execute stories in parallel
            let semaphore = Arc::new(tokio::sync::Semaphore::new(
                self.config.max_concurrent_stories,
            ));
            let provider = Arc::clone(&self.provider);
            let model = self.model.clone();
            let prd_info = (
                self.state.prd.project.clone(),
                self.state.prd.feature.clone(),
            );
            let working_dir = self.state.working_dir.clone();
            let progress_path = self.config.progress_path.clone();

            let mut handles = Vec::new();

            // Collect accumulated learnings from previous stages via bus
            let accumulated_learnings = self.bus_collect_learnings();

            for story in stories {
                let sem = Arc::clone(&semaphore);
                let provider = Arc::clone(&provider);
                let model = model.clone();
                let prd_info = prd_info.clone();
                let working_dir = working_dir.clone();
                let worktree_mgr = worktree_mgr.clone();
                let prebuilt_worktree = worktrees_by_story.remove(&story.id);
                let progress_path = progress_path.clone();
                let ralph_tx = self.event_tx.clone();
                let stage_learnings = accumulated_learnings.clone();
                let bus = self.bus.clone();
                let relay_enabled = self.config.relay_enabled;
                let relay_max_agents = self.config.relay_max_agents;
                let relay_max_rounds = self.config.relay_max_rounds;
                let registry = self.registry.clone();
                let max_steps_per_story = self.config.max_steps_per_story;
                let max_quality_retries = self.config.max_quality_retries;
                let max_rate_limit_retries = self.config.max_rate_limit_retries;
                let rate_limit_base_delay_ms = self.config.rate_limit_base_delay_ms;
                let quality_checks_enabled = self.config.quality_checks_enabled;
                let quality_checks = self.state.prd.quality_checks.clone();
                let delegation = self.delegation.clone();

                let handle: tokio::task::JoinHandle<(
                    crate::ralph::types::UserStory,
                    bool,
                    crate::ralph::types::ProgressEntry,
                    Option<crate::worktree::WorktreeInfo>,
                    Option<std::sync::Arc<crate::worktree::WorktreeManager>>,
                    bool, // quality_passed
                    Option<String>,
                )> = tokio::spawn(async move {
                    let _permit = sem.acquire().await.expect("semaphore closed");

                    // Use the pre-created worktree for this story if one
                    // was provisioned above; otherwise fall back to the
                    // main working directory. Worktree creation now
                    // happens in a bounded parallel pass *before* task
                    // spawn, so we no longer race the `.git/worktrees`
                    // lock inside every task.
                    let (story_working_dir, worktree_info) = if let Some(wt) = prebuilt_worktree {
                        (wt.path.clone(), Some(wt))
                    } else {
                        (working_dir.clone(), None)
                    };

                    info!(
                        "Working on story: {} - {} (in {:?})",
                        story.id, story.title, story_working_dir
                    );

                    // Emit StoryStarted event
                    if let Some(ref tx) = ralph_tx {
                        let _ = tx
                            .send(RalphEvent::StoryStarted {
                                story_id: story.id.clone(),
                            })
                            .await;
                    }

                    // Build the prompt with worktree awareness + accumulated learnings
                    let mut prompt =
                        Self::build_story_prompt(&story, &prd_info, &story_working_dir);
                    if !stage_learnings.is_empty() {
                        prompt.push_str("\n## Learnings from Previous Stages:\n");
                        for l in &stage_learnings {
                            prompt.push_str(&format!("- {l}\n"));
                        }
                    }

                    // Create event bridge for this story's sub-agent (used for single-agent mode)
                    let (bridge_tx, _bridge_handle) = if !relay_enabled {
                        if let Some(ref tx) = ralph_tx {
                            let (btx, handle) =
                                Self::create_swarm_event_bridge(tx, story.id.clone());
                            (Some(btx), Some(handle))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    };

                    // Execute story: relay team or single agent, with rate-limit retry
                    let call_result = {
                        let mut last_err = None;
                        let mut attempt_result = None;
                        for attempt in 0..=max_rate_limit_retries {
                            let res = if relay_enabled {
                                if let Some(ref reg) = registry {
                                    Self::call_relay_static(
                                        reg,
                                        &model,
                                        &prompt,
                                        &story_working_dir,
                                        ralph_tx.clone(),
                                        story.id.clone(),
                                        bus.clone(),
                                        relay_max_agents,
                                        relay_max_rounds,
                                        delegation.as_ref(),
                                    )
                                    .await
                                } else {
                                    warn!(
                                        story_id = %story.id,
                                        "Relay enabled but no registry available, using single agent"
                                    );
                                    Self::call_llm_static(
                                        &provider,
                                        &model,
                                        &prompt,
                                        &story_working_dir,
                                        bridge_tx.clone(),
                                        story.id.clone(),
                                        bus.clone(),
                                        max_steps_per_story,
                                    )
                                    .await
                                }
                            } else {
                                Self::call_llm_static(
                                    &provider,
                                    &model,
                                    &prompt,
                                    &story_working_dir,
                                    bridge_tx.clone(),
                                    story.id.clone(),
                                    bus.clone(),
                                    max_steps_per_story,
                                )
                                .await
                            };
                            match res {
                                Ok(output) => {
                                    attempt_result = Some(Ok(output));
                                    break;
                                }
                                Err(e) => {
                                    let err_str = format!("{e}");
                                    let is_rate_limit = err_str.contains("Rate limit")
                                        || err_str.contains("rate_limit")
                                        || err_str.contains("429")
                                        || err_str.contains("too many requests");
                                    if is_rate_limit && attempt < max_rate_limit_retries {
                                        let delay_ms = rate_limit_base_delay_ms
                                            * 2u64.saturating_pow(attempt as u32);
                                        warn!(
                                            story_id = %story.id,
                                            attempt = attempt + 1,
                                            max_retries = max_rate_limit_retries,
                                            delay_ms = delay_ms,
                                            "Rate limited, backing off before retry"
                                        );
                                        tokio::time::sleep(std::time::Duration::from_millis(
                                            delay_ms,
                                        ))
                                        .await;
                                    } else {
                                        last_err = Some(e);
                                        break;
                                    }
                                }
                            }
                        }
                        attempt_result.unwrap_or_else(|| {
                            Err(last_err
                                .unwrap_or_else(|| anyhow::anyhow!("All retries exhausted")))
                        })
                    };

                    // Quality-gate retry loop: if agent succeeded but quality gates fail,
                    // feed errors back and let the agent try to fix them.
                    let (final_result, quality_passed, failure_reason) = match call_result {
                        Ok(initial_output) => {
                            let mut output = initial_output;
                            let mut qg_passed = false;
                            let mut failure_reason = None;

                            if quality_checks_enabled {
                                for qg_attempt in 0..=max_quality_retries {
                                    let cargo_root = Self::find_cargo_root(&story_working_dir);
                                    let checks = &quality_checks;
                                    let mut all_ok = true;
                                    let mut error_output = String::new();

                                    for (name, cmd) in [
                                        ("typecheck", &checks.typecheck),
                                        ("lint", &checks.lint),
                                        ("test", &checks.test),
                                        ("build", &checks.build),
                                    ] {
                                        if let Some(command) = cmd {
                                            let cmd_output =
                                                quality_shell::run(&cargo_root, command).await;
                                            match cmd_output {
                                                Ok(o) if o.success => {}
                                                Ok(o) => {
                                                    // Extract error lines for agent feedback
                                                    let errors: String = o
                                                        .combined
                                                        .lines()
                                                        .filter(|l| {
                                                            l.starts_with("error")
                                                                || l.contains("error:")
                                                                || l.contains("error[")
                                                        })
                                                        .take(20)
                                                        .collect::<Vec<_>>()
                                                        .join("\n");
                                                    error_output = format!(
                                                        "Quality check `{name}` failed:\n{errors}"
                                                    );
                                                    all_ok = false;
                                                    break;
                                                }
                                                Err(e) => {
                                                    error_output = format!(
                                                        "Quality check `{name}` could not run: {e}"
                                                    );
                                                    all_ok = false;
                                                    break;
                                                }
                                            }
                                        }
                                    }

                                    if all_ok {
                                        match run_story_verification(&story_working_dir, &story)
                                            .await
                                        {
                                            Ok(()) => {
                                                qg_passed = true;
                                                failure_reason = None;
                                                info!(
                                                    story_id = %story.id,
                                                    attempt = qg_attempt + 1,
                                                    "Quality gates and verification passed"
                                                );
                                                break;
                                            }
                                            Err(e) => {
                                                error_output =
                                                    format!("Verification steps failed:\n{e}");
                                                failure_reason = Some(error_output.clone());
                                            }
                                        }
                                    }

                                    if qg_attempt < max_quality_retries {
                                        if !error_output.is_empty() {
                                            failure_reason = Some(error_output.clone());
                                        }
                                        // Feed errors back to agent for self-repair
                                        warn!(
                                            story_id = %story.id,
                                            attempt = qg_attempt + 1,
                                            max_retries = max_quality_retries,
                                            "Quality gates failed, spawning repair agent"
                                        );

                                        let repair_prompt = format!(
                                            "# QUALITY GATE REPAIR (attempt {}/{})\n\n\
                                             The previous implementation attempt for story {} had quality gate failures.\n\n\
                                             ## Errors:\n```\n{}\n```\n\n\
                                             ## Instructions:\n\
                                             1. Read the files mentioned in the errors\n\
                                             2. Fix ONLY the compile/lint errors shown above\n\
                                             3. Run `cargo check 2>&1` to verify your fix\n\
                                             4. Do NOT refactor or change unrelated code\n\n\
                                             Working directory: {}",
                                            qg_attempt + 1,
                                            max_quality_retries,
                                            story.id,
                                            error_output,
                                            story_working_dir.display()
                                        );

                                        // Repair agent gets fewer steps
                                        let repair_result = Self::call_llm_static(
                                            &provider,
                                            &model,
                                            &repair_prompt,
                                            &story_working_dir,
                                            bridge_tx.clone(),
                                            story.id.clone(),
                                            bus.clone(),
                                            20, // fewer steps for targeted repair
                                        )
                                        .await;

                                        match repair_result {
                                            Ok(repair_output) => {
                                                output = format!(
                                                    "{output}\n---\n[repair attempt {}] {repair_output}",
                                                    qg_attempt + 1
                                                );
                                            }
                                            Err(e) => {
                                                warn!(
                                                    story_id = %story.id,
                                                    error = %e,
                                                    "Repair agent failed"
                                                );
                                                break;
                                            }
                                        }
                                    } else {
                                        if !error_output.is_empty() {
                                            failure_reason = Some(error_output.clone());
                                        }
                                        warn!(
                                            story_id = %story.id,
                                            "Quality gates still failing after {} retries",
                                            max_quality_retries
                                        );
                                    }
                                }
                            } else {
                                match run_story_verification(&story_working_dir, &story).await {
                                    Ok(()) => {
                                        qg_passed = true;
                                        failure_reason = None;
                                    }
                                    Err(e) => {
                                        failure_reason =
                                            Some(format!("Verification steps failed:\n{e}"));
                                    }
                                }
                            }

                            (Ok(output), qg_passed, failure_reason)
                        }
                        Err(e) => (Err(e), false, None),
                    };

                    let entry = match &final_result {
                        Ok(response) => {
                            // Append progress to worktree-local progress file
                            let progress_file = story_working_dir.join(&progress_path);
                            let _ = std::fs::write(&progress_file, response);

                            ProgressEntry {
                                story_id: story.id.clone(),
                                iteration: 1,
                                status: "completed".to_string(),
                                learnings: Self::extract_learnings_static(response),
                                files_changed: Vec::new(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            }
                        }
                        Err(e) => {
                            warn!("LLM call failed for story {}: {}", story.id, e);
                            ProgressEntry {
                                story_id: story.id.clone(),
                                iteration: 1,
                                status: format!("failed: {}", e),
                                learnings: Vec::new(),
                                files_changed: Vec::new(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            }
                        }
                    };

                    (
                        story,
                        final_result.is_ok(),
                        entry,
                        worktree_info,
                        worktree_mgr,
                        quality_passed,
                        failure_reason,
                    )
                });

                handles.push(handle);
            }

            // Wait for all stories in this stage
            let _stage_passed = 0usize;
            for handle in handles {
                match handle.await {
                    Ok((
                        story,
                        success,
                        entry,
                        worktree_info,
                        worktree_mgr,
                        quality_passed,
                        failure_reason,
                    )) => {
                        self.state.progress_log.push(entry);

                        if success && quality_passed {
                            // Quality gates already ran (with retry/repair) inside the spawned task.
                            // Proceed directly to merge.
                            {
                                info!("Story {} passed quality checks!", story.id);

                                // Commit in worktree first
                                if let Some(ref wt) = worktree_info {
                                    let _ = self.commit_in_dir(&wt.path, &story);
                                }

                                // Merge worktree back to main
                                if let (Some(wt), Some(mgr)) =
                                    (worktree_info.as_ref(), worktree_mgr.as_ref())
                                {
                                    match mgr.merge(&wt.name).await {
                                        Ok(merge_result) => {
                                            if merge_result.success {
                                                info!(
                                                    story_id = %story.id,
                                                    files_changed = merge_result.files_changed,
                                                    "Merged story changes successfully"
                                                );
                                                if let Err(error) =
                                                    run_story_verification(&working_dir, &story)
                                                        .await
                                                {
                                                    let error = format!(
                                                        "Verification steps failed after merge: {error}"
                                                    );
                                                    warn!(story_id = %story.id, error = %error);
                                                    self.record_story_result(
                                                        &story,
                                                        false,
                                                        Some(error),
                                                    );
                                                    self.try_send_event(
                                                        RalphEvent::StoryComplete {
                                                            story_id: story.id.clone(),
                                                            passed: false,
                                                        },
                                                    );
                                                    let _ = mgr.cleanup(&wt.name).await;
                                                    continue;
                                                }
                                                self.state.prd.mark_passed(&story.id);
                                                self.try_send_event(RalphEvent::StoryMerge {
                                                    story_id: story.id.clone(),
                                                    success: true,
                                                    summary: merge_result.summary.clone(),
                                                });
                                                self.try_send_event(RalphEvent::StoryComplete {
                                                    story_id: story.id.clone(),
                                                    passed: true,
                                                });

                                                // Record story result in store
                                                self.record_story_result(&story, true, None);

                                                // Cleanup worktree
                                                let _ = mgr.cleanup(&wt.name).await;
                                            } else if !merge_result.conflicts.is_empty() {
                                                // Real conflicts - spawn conflict resolver
                                                info!(
                                                    story_id = %story.id,
                                                    num_conflicts = merge_result.conflicts.len(),
                                                    "Spawning conflict resolver sub-agent"
                                                );

                                                // Try to resolve conflicts
                                                match Self::resolve_conflicts_static(
                                                    &provider,
                                                    &model,
                                                    &working_dir,
                                                    &story,
                                                    &merge_result.conflicts,
                                                    &merge_result.conflict_diffs,
                                                    self.bus.clone(),
                                                )
                                                .await
                                                {
                                                    Ok(resolved) => {
                                                        if resolved {
                                                            // Complete the merge after resolution
                                                            let commit_msg = format!(
                                                                "Merge: resolved conflicts for {}",
                                                                story.id
                                                            );
                                                            match mgr
                                                                .complete_merge(
                                                                    &wt.name,
                                                                    &commit_msg,
                                                                )
                                                                .await
                                                            {
                                                                Ok(final_result) => {
                                                                    if final_result.success {
                                                                        info!(
                                                                            story_id = %story.id,
                                                                            "Merge completed after conflict resolution"
                                                                        );
                                                                        match run_story_verification(
                                                                            &working_dir,
                                                                            &story,
                                                                        )
                                                                        .await
                                                                        {
                                                                            Ok(()) => self
                                                                                .state
                                                                                .prd
                                                                                .mark_passed(
                                                                                    &story.id,
                                                                                ),
                                                                            Err(e) => {
                                                                                let error = format!(
                                                                                    "Verification failed after conflict resolution: {e}"
                                                                                );
                                                                                warn!(
                                                                                    story_id = %story.id,
                                                                                    error = %error
                                                                                );
                                                                                self.record_story_result(
                                                                                    &story,
                                                                                    false,
                                                                                    Some(error),
                                                                                );
                                                                                self.try_send_event(RalphEvent::StoryComplete {
                                                                                    story_id: story.id.clone(),
                                                                                    passed: false,
                                                                                });
                                                                            }
                                                                        }
                                                                    } else {
                                                                        warn!(
                                                                            story_id = %story.id,
                                                                            "Merge failed even after resolution"
                                                                        );
                                                                        let _ = mgr
                                                                            .abort_merge(&wt.name)
                                                                            .await;
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    warn!(
                                                                        story_id = %story.id,
                                                                        error = %e,
                                                                        "Failed to complete merge after resolution"
                                                                    );
                                                                    let _ = mgr
                                                                        .abort_merge(&wt.name)
                                                                        .await;
                                                                }
                                                            }
                                                        } else {
                                                            warn!(
                                                                story_id = %story.id,
                                                                "Conflict resolver could not resolve all conflicts"
                                                            );
                                                            let _ = mgr.abort_merge(&wt.name).await;
                                                        }
                                                    }
                                                    Err(e) => {
                                                        warn!(
                                                            story_id = %story.id,
                                                            error = %e,
                                                            "Conflict resolver failed"
                                                        );
                                                        let _ = mgr.abort_merge(&wt.name).await;
                                                    }
                                                }
                                                // Cleanup worktree
                                                let _ = mgr.cleanup(&wt.name).await;
                                            } else if merge_result.aborted {
                                                // Non-conflict failure that was already aborted
                                                warn!(
                                                    story_id = %story.id,
                                                    summary = %merge_result.summary,
                                                    "Merge was aborted due to non-conflict failure"
                                                );
                                                // Cleanup worktree
                                                let _ = mgr.cleanup(&wt.name).await;
                                            } else {
                                                // Merge in progress state (should not reach here)
                                                warn!(
                                                    story_id = %story.id,
                                                    summary = %merge_result.summary,
                                                    "Merge failed but not aborted - manual intervention may be needed"
                                                );
                                                // Don't cleanup - leave for debugging
                                            }
                                        }
                                        Err(e) => {
                                            warn!(
                                                story_id = %story.id,
                                                error = %e,
                                                "Failed to merge worktree"
                                            );
                                        }
                                    }
                                } else {
                                    // No worktree; verification still gates completion.
                                    if let Err(error) =
                                        run_story_verification(&working_dir, &story).await
                                    {
                                        let error = format!("Verification steps failed: {error}");
                                        warn!(story_id = %story.id, error = %error);
                                        self.record_story_result(&story, false, Some(error));
                                        self.try_send_event(RalphEvent::StoryComplete {
                                            story_id: story.id.clone(),
                                            passed: false,
                                        });
                                        continue;
                                    }
                                    self.state.prd.mark_passed(&story.id);
                                    self.try_send_event(RalphEvent::StoryComplete {
                                        story_id: story.id.clone(),
                                        passed: true,
                                    });

                                    // Record story result in store
                                    self.record_story_result(&story, true, None);
                                }
                            }
                        } else {
                            // Failed - cleanup worktree without merging (keep for debugging)
                            let error = failure_reason.unwrap_or_else(|| {
                                if success {
                                    "Quality or verification checks failed".to_string()
                                } else {
                                    "LLM call failed".to_string()
                                }
                            });
                            self.try_send_event(RalphEvent::StoryError {
                                story_id: story.id.clone(),
                                error: error.clone(),
                            });
                            self.record_story_result(&story, false, Some(error));
                            if let Some(ref wt) = worktree_info {
                                info!(
                                    story_id = %story.id,
                                    worktree_path = %wt.path.display(),
                                    "Keeping worktree for debugging (story failed)"
                                );
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Story execution task failed: {}", e);
                    }
                }
            }

            // Increment iteration once per stage, not per story
            self.state.current_iteration += 1;

            // Publish stage-level progress to bus after all stories in this stage
            if self.bus.is_some() {
                let bus_learnings: Vec<String> = self
                    .state
                    .progress_log
                    .iter()
                    .flat_map(|e| e.learnings.clone())
                    .collect();
                for story in &self.state.prd.user_stories {
                    if story.passes {
                        self.bus_publish_story_result(
                            story,
                            self.state.current_iteration,
                            &bus_learnings,
                            None,
                        );
                    }
                }
            }

            // Save PRD after each stage
            self.state.prd.save(&self.state.prd_path).await?;

            // Persist PRD + iteration to store after each stage
            if let Some(ref store) = self.store {
                let s = store.clone();
                let rid = self.run_id.clone();
                let prd = self.state.prd.clone();
                let iter = self.state.current_iteration;
                self.store_fire_and_forget(async move {
                    s.update_prd(&rid, &prd).await?;
                    s.update_iteration(&rid, iter).await
                });
            }
        }

        Ok(())
    }

    /// Call LLM with agentic tool loop (static version for parallel execution)
    async fn call_llm_static(
        provider: &Arc<dyn Provider>,
        model: &str,
        prompt: &str,
        working_dir: &PathBuf,
        event_tx: Option<mpsc::Sender<SwarmEvent>>,
        story_id: String,
        bus: Option<Arc<AgentBus>>,
        max_steps_per_story: usize,
    ) -> anyhow::Result<String> {
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(working_dir);

        // Create tool registry with provider for file operations
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(provider), model.to_string());

        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(
            "Ralph sub-agent starting with {} tools in {:?}",
            tool_definitions.len(),
            working_dir
        );

        // Run the agentic loop with tools
        let max_steps = max_steps_per_story;
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
            Arc::clone(provider),
            model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry, // Already an Arc<ToolRegistry>
            max_steps,
            180, // 3 minute timeout per story
            event_tx,
            story_id,
            bus.clone(),
            Some(working_dir.clone()),
        )
        .await?;

        info!(
            "Ralph sub-agent completed: {} steps, {} tool calls",
            steps, tool_calls
        );

        Ok(output)
    }

    /// Build default relay profiles for code implementation tasks.
    fn build_implementation_relay_profiles(
        max_agents: usize,
        working_dir: &std::path::Path,
    ) -> Vec<(String, String, Vec<String>)> {
        codetether_ralph_core::relay_profiles::build_implementation_relay_profiles(
            max_agents,
            working_dir,
        )
    }

    /// Call the LLM with a prompt using agentic tool loop
    async fn call_llm(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(&self.state.working_dir);

        // Create tool registry with provider for file operations
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(&self.provider), self.model.clone());

        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(
            "Ralph agent starting with {} tools in {:?}",
            tool_definitions.len(),
            self.state.working_dir
        );

        // Create event bridge if we have an event channel
        let (bridge_tx, _bridge_handle) = if let Some(ref ralph_tx) = self.event_tx {
            let (tx, handle) = Self::create_swarm_event_bridge(ralph_tx, story_id.to_string());
            (Some(tx), Some(handle))
        } else {
            (None, None)
        };

        // Run the agentic loop with tools
        let (output, steps, tool_calls, exit_reason) = run_agent_loop(
            Arc::clone(&self.provider),
            &self.model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry, // Already an Arc<ToolRegistry>
            30,            // max steps per story (focused implementation)
            180,           // 3 minute timeout per story
            bridge_tx,
            story_id.to_string(),
            self.bus.clone(),
            Some(self.state.working_dir.clone()),
        )
        .await?;

        // Check exit reason and log appropriately
        match exit_reason {
            AgentLoopExit::Completed => {
                info!(
                    "Ralph agent completed: {} steps, {} tool calls",
                    steps, tool_calls
                );
            }
            AgentLoopExit::MaxStepsReached => {
                warn!(
                    story_id = %story_id,
                    steps = steps,
                    tool_calls = tool_calls,
                    "Ralph sub-agent hit max steps limit - work may be incomplete"
                );
            }
            AgentLoopExit::TimedOut => {
                warn!(
                    story_id = %story_id,
                    steps = steps,
                    tool_calls = tool_calls,
                    "Ralph sub-agent timed out - work may be incomplete"
                );
            }
        }

        Ok(output)
    }
}

/// Create a sample PRD template
pub fn create_prd_template(project: &str, feature: &str) -> Prd {
    codetether_ralph_core::create_prd_template(project, feature)
}
