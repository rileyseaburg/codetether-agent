//! Ralph loop - the core autonomous execution loop

use super::types::*;
use crate::bus::AgentBus;
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::provider::{self, CompletionRequest, ContentPart, Message, Provider, ProviderRegistry, Role};
use crate::session::{Session, SessionEvent};
use crate::swarm::run_agent_loop;
use std::collections::HashMap;
use crate::tool::ToolRegistry;
use crate::tui::ralph_view::{RalphEvent, RalphStoryInfo, RalphStoryStatus};
use crate::tui::swarm_view::SwarmEvent;
use crate::worktree::WorktreeManager;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// The main Ralph executor
pub struct RalphLoop {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
    event_tx: Option<mpsc::Sender<RalphEvent>>,
    bus: Option<Arc<AgentBus>>,
    registry: Option<Arc<ProviderRegistry>>,
}

impl RalphLoop {
    /// Create a new Ralph loop
    pub async fn new(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> anyhow::Result<Self> {
        let prd = Prd::load(&prd_path).await?;

        // Get working directory - use parent of prd_path or current directory
        let working_dir = if let Some(parent) = prd_path.parent() {
            if parent.as_os_str().is_empty() {
                std::env::current_dir()?
            } else {
                parent.to_path_buf()
            }
        } else {
            std::env::current_dir()?
        };

        info!(
            "Loaded PRD: {} - {} ({} stories)",
            prd.project,
            prd.feature,
            prd.user_stories.len()
        );

        let state = RalphState {
            prd,
            current_iteration: 0,
            max_iterations: config.max_iterations,
            status: RalphStatus::Pending,
            progress_log: Vec::new(),
            prd_path: prd_path.clone(),
            working_dir,
        };

        Ok(Self {
            state,
            provider,
            model,
            config,
            event_tx: None,
            bus: None,
            registry: None,
        })
    }

    /// Attach an event channel for TUI updates
    pub fn with_event_tx(mut self, tx: mpsc::Sender<RalphEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Attach an agent bus for inter-iteration learning and context sharing
    pub fn with_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Attach a provider registry for relay team planning
    pub fn with_registry(mut self, registry: Arc<ProviderRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Non-blocking send of a Ralph event
    fn try_send_event(&self, event: RalphEvent) {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.try_send(event);
        }
    }

    /// Publish story learnings, handoff context, and progress to the bus.
    fn bus_publish_story_result(
        &self,
        story: &UserStory,
        iteration: usize,
        learnings: &[String],
        next_story_id: Option<&str>,
    ) {
        let Some(ref bus) = self.bus else { return };
        let prd_id = &self.state.prd.project;
        let handle = bus.handle(format!("ralph.{}", story.id));

        // Publish learnings
        handle.publish_ralph_learning(
            prd_id,
            &story.id,
            iteration,
            learnings.to_vec(),
            serde_json::json!({
                "story_title": story.title,
                "passed": story.passes,
            }),
        );

        // Publish handoff to next story if applicable
        if let Some(next_id) = next_story_id {
            handle.publish_ralph_handoff(
                prd_id,
                &story.id,
                next_id,
                serde_json::json!({ "learnings": learnings }),
                &format!(
                    "{} {} (iteration {})",
                    story.id,
                    if story.passes { "passed" } else { "failed" },
                    iteration
                ),
            );
        }

        // Publish progress
        handle.publish_ralph_progress(
            prd_id,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len(),
            iteration,
            &format!("{:?}", self.state.status),
        );
    }

    /// Collect accumulated learnings from the bus for injection into prompts.
    fn bus_collect_learnings(&self) -> Vec<String> {
        let Some(ref bus) = self.bus else {
            return Vec::new();
        };
        let prd_id = &self.state.prd.project;
        let mut handle = bus.handle("ralph-collector");
        let envelopes = handle.drain_ralph_learnings(prd_id);
        let mut learnings = Vec::new();
        for env in envelopes {
            match env.message {
                crate::bus::BusMessage::RalphLearning {
                    story_id,
                    iteration,
                    learnings: items,
                    ..
                } => {
                    for l in items {
                        learnings.push(format!("[{story_id} iter {iteration}] {l}"));
                    }
                }
                crate::bus::BusMessage::RalphHandoff {
                    from_story,
                    progress_summary,
                    ..
                } => {
                    learnings.push(format!("[handoff from {from_story}] {progress_summary}"));
                }
                _ => {}
            }
        }
        learnings
    }

    /// Create a bridge that forwards SwarmEvent → RalphEvent for a given story_id.
    /// Returns the sender to pass to `run_agent_loop` and a join handle for the
    /// forwarding task.
    fn create_swarm_event_bridge(
        ralph_tx: &mpsc::Sender<RalphEvent>,
        story_id: String,
    ) -> (mpsc::Sender<SwarmEvent>, tokio::task::JoinHandle<()>) {
        let (swarm_tx, mut swarm_rx) = mpsc::channel::<SwarmEvent>(100);
        let ralph_tx = ralph_tx.clone();
        let handle = tokio::spawn(async move {
            while let Some(event) = swarm_rx.recv().await {
                let ralph_event = match event {
                    SwarmEvent::AgentToolCall { tool_name, .. } => RalphEvent::StoryToolCall {
                        story_id: story_id.clone(),
                        tool_name,
                    },
                    SwarmEvent::AgentToolCallDetail { detail, .. } => {
                        RalphEvent::StoryToolCallDetail {
                            story_id: story_id.clone(),
                            detail,
                        }
                    }
                    SwarmEvent::AgentMessage { entry, .. } => RalphEvent::StoryMessage {
                        story_id: story_id.clone(),
                        entry,
                    },
                    SwarmEvent::AgentOutput { output, .. } => RalphEvent::StoryOutput {
                        story_id: story_id.clone(),
                        output,
                    },
                    SwarmEvent::AgentError { error, .. } => RalphEvent::StoryError {
                        story_id: story_id.clone(),
                        error,
                    },
                    _ => continue, // Skip swarm-specific events
                };
                if ralph_tx.send(ralph_event).await.is_err() {
                    break;
                }
            }
        });
        (swarm_tx, handle)
    }

    /// Build initial RalphStoryInfo list from the PRD
    fn build_story_infos(prd: &Prd) -> Vec<RalphStoryInfo> {
        prd.user_stories
            .iter()
            .map(|s| RalphStoryInfo {
                id: s.id.clone(),
                title: s.title.clone(),
                status: if s.passes {
                    RalphStoryStatus::Passed
                } else {
                    RalphStoryStatus::Pending
                },
                priority: s.priority,
                depends_on: s.depends_on.clone(),
                quality_checks: Vec::new(),
                tool_call_history: Vec::new(),
                messages: Vec::new(),
                output: None,
                error: None,
                merge_summary: None,
                steps: 0,
                current_tool: None,
            })
            .collect()
    }

    /// Run the Ralph loop until completion or max iterations
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        self.state.status = RalphStatus::Running;

        // Emit Started event
        self.try_send_event(RalphEvent::Started {
            project: self.state.prd.project.clone(),
            feature: self.state.prd.feature.clone(),
            stories: Self::build_story_infos(&self.state.prd),
            max_iterations: self.state.max_iterations,
        });

        // Switch to feature branch
        if !self.state.prd.branch_name.is_empty() {
            info!("Switching to branch: {}", self.state.prd.branch_name);
            self.git_checkout(&self.state.prd.branch_name)?;
        }

        // Choose execution mode
        if self.config.parallel_enabled {
            self.run_parallel().await?;
        } else {
            self.run_sequential().await?;
        }

        if self.state.status != RalphStatus::Completed
            && self.state.current_iteration >= self.state.max_iterations
        {
            self.state.status = RalphStatus::MaxIterations;
        }

        // Clean up orphaned worktrees and branches
        if self.config.worktree_enabled {
            if let Ok(mgr) = WorktreeManager::new(&self.state.working_dir) {
                match mgr.cleanup_all() {
                    Ok(count) if count > 0 => {
                        info!(cleaned = count, "Cleaned up orphaned worktrees/branches");
                    }
                    Ok(_) => {}
                    Err(e) => {
                        warn!(error = %e, "Failed to cleanup orphaned worktrees");
                    }
                }
            }
        }

        info!(
            "Ralph finished: {:?}, {}/{} stories passed",
            self.state.status,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len()
        );

        // Emit Complete event
        self.try_send_event(RalphEvent::Complete {
            status: format!("{:?}", self.state.status),
            passed: self.state.prd.passed_count(),
            total: self.state.prd.user_stories.len(),
        });

        Ok(self.state.clone())
    }

    /// Run stories sequentially (original behavior)
    async fn run_sequential(&mut self) -> anyhow::Result<()> {
        while self.state.current_iteration < self.state.max_iterations {
            self.state.current_iteration += 1;
            info!(
                "=== Ralph iteration {} of {} ===",
                self.state.current_iteration, self.state.max_iterations
            );

            // Emit iteration event
            self.try_send_event(RalphEvent::IterationStarted {
                iteration: self.state.current_iteration,
                max_iterations: self.state.max_iterations,
            });

            // Check if all stories are complete
            if self.state.prd.is_complete() {
                info!("All stories complete!");
                self.state.status = RalphStatus::Completed;
                break;
            }

            // Get next story to work on
            let story = match self.state.prd.next_story() {
                Some(s) => s.clone(),
                None => {
                    warn!("No available stories (dependencies not met)");
                    break;
                }
            };

            info!("Working on story: {} - {}", story.id, story.title);

            // Emit StoryStarted event
            self.try_send_event(RalphEvent::StoryStarted {
                story_id: story.id.clone(),
            });

            // Collect accumulated bus learnings for prompt injection
            let bus_learnings = self.bus_collect_learnings();

            // Build the prompt (with bus learnings if available)
            let prompt = if bus_learnings.is_empty() {
                self.build_prompt(&story)
            } else {
                let mut p = self.build_prompt(&story);
                p.push_str("\n## Learnings from Previous Iterations:\n");
                for l in &bus_learnings {
                    p.push_str(&format!("- {l}\n"));
                }
                p
            };

            // Call the LLM
            match self.call_llm(&story.id, &prompt).await {
                Ok(response) => {
                    // Log progress
                    let entry = ProgressEntry {
                        story_id: story.id.clone(),
                        iteration: self.state.current_iteration,
                        status: "completed".to_string(),
                        learnings: self.extract_learnings(&response),
                        files_changed: Vec::new(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };
                    self.append_progress(&entry, &response)?;
                    let entry_learnings = entry.learnings.clone();
                    self.state.progress_log.push(entry);

                    // Run quality gates
                    if self.config.quality_checks_enabled {
                        if self.run_quality_gates_with_events(&story.id).await? {
                            info!("Story {} passed quality checks!", story.id);
                            self.state.prd.mark_passed(&story.id);

                            self.try_send_event(RalphEvent::StoryComplete {
                                story_id: story.id.clone(),
                                passed: true,
                            });

                            // Publish learnings to bus
                            let next = self.state.prd.next_story().map(|s| s.id.clone());
                            self.bus_publish_story_result(
                                &story,
                                self.state.current_iteration,
                                &entry_learnings,
                                next.as_deref(),
                            );

                            // Commit changes
                            if self.config.auto_commit {
                                self.commit_story(&story)?;
                            }

                            // Save updated PRD
                            self.state.prd.save(&self.state.prd_path).await?;
                        } else {
                            warn!("Story {} failed quality checks", story.id);

                            // Publish failure learnings to bus
                            let next = self.state.prd.next_story().map(|s| s.id.clone());
                            self.bus_publish_story_result(
                                &story,
                                self.state.current_iteration,
                                &entry_learnings,
                                next.as_deref(),
                            );

                            self.try_send_event(RalphEvent::StoryComplete {
                                story_id: story.id.clone(),
                                passed: false,
                            });
                        }
                    } else {
                        // No quality checks, just mark as passed
                        self.state.prd.mark_passed(&story.id);
                        self.state.prd.save(&self.state.prd_path).await?;

                        self.try_send_event(RalphEvent::StoryComplete {
                            story_id: story.id.clone(),
                            passed: true,
                        });
                    }
                }
                Err(e) => {
                    warn!("LLM call failed: {}", e);
                    let entry = ProgressEntry {
                        story_id: story.id.clone(),
                        iteration: self.state.current_iteration,
                        status: format!("failed: {}", e),
                        learnings: Vec::new(),
                        files_changed: Vec::new(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };
                    self.state.progress_log.push(entry);

                    self.try_send_event(RalphEvent::StoryError {
                        story_id: story.id.clone(),
                        error: format!("{}", e),
                    });
                }
            }
        }

        Ok(())
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
            match WorktreeManager::new(&self.state.working_dir) {
                Ok(mgr) => {
                    info!("Worktree isolation enabled for parallel stories");
                    Some(Arc::new(mgr))
                }
                Err(e) => {
                    warn!(
                        "Failed to create worktree manager: {}, falling back to sequential within stages",
                        e
                    );
                    None
                }
            }
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
                let progress_path = progress_path.clone();
                let ralph_tx = self.event_tx.clone();
                let stage_learnings = accumulated_learnings.clone();
                let bus = self.bus.clone();
                let relay_enabled = self.config.relay_enabled;
                let relay_max_agents = self.config.relay_max_agents;
                let relay_max_rounds = self.config.relay_max_rounds;
                let registry = self.registry.clone();

                let handle = tokio::spawn(async move {
                    let _permit = sem.acquire().await.expect("semaphore closed");

                    // Create worktree for this story if enabled
                    let (story_working_dir, worktree_info) = if let Some(ref mgr) = worktree_mgr {
                        match mgr.create(&story.id.to_lowercase().replace("-", "_")) {
                            Ok(wt) => {
                                // Inject [workspace] stub for hermetic isolation
                                if let Err(e) = mgr.inject_workspace_stub(&wt.path) {
                                    warn!(
                                        story_id = %story.id,
                                        error = %e,
                                        "Failed to inject workspace stub"
                                    );
                                }
                                info!(
                                    story_id = %story.id,
                                    worktree_path = %wt.path.display(),
                                    "Created worktree for story"
                                );
                                (wt.path.clone(), Some(wt))
                            }
                            Err(e) => {
                                warn!(
                                    story_id = %story.id,
                                    error = %e,
                                    "Failed to create worktree, using main directory"
                                );
                                (working_dir.clone(), None)
                            }
                        }
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
                            let (btx, handle) = Self::create_swarm_event_bridge(tx, story.id.clone());
                            (Some(btx), Some(handle))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    };

                    // Execute story: relay team or single agent
                    let result = if relay_enabled {
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
                                bridge_tx,
                                story.id.clone(),
                                bus.clone(),
                            )
                            .await
                        }
                    } else {
                        Self::call_llm_static(
                            &provider,
                            &model,
                            &prompt,
                            &story_working_dir,
                            bridge_tx,
                            story.id.clone(),
                            bus.clone(),
                        )
                        .await
                    };

                    let entry = match &result {
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

                    (story, result.is_ok(), entry, worktree_info, worktree_mgr)
                });

                handles.push(handle);
            }

            // Wait for all stories in this stage
            for handle in handles {
                match handle.await {
                    Ok((story, success, entry, worktree_info, worktree_mgr)) => {
                        self.state.current_iteration += 1;
                        self.state.progress_log.push(entry);

                        if success {
                            // Run quality gates in the worktree (or main dir)
                            let check_dir = worktree_info
                                .as_ref()
                                .map(|wt| wt.path.clone())
                                .unwrap_or_else(|| self.state.working_dir.clone());

                            let quality_passed = if self.config.quality_checks_enabled {
                                self.run_quality_gates_in_dir_with_events(&check_dir, &story.id)
                                    .await
                                    .unwrap_or(false)
                            } else {
                                true
                            };

                            if quality_passed {
                                info!("Story {} passed quality checks!", story.id);

                                // Commit in worktree first
                                if let Some(ref wt) = worktree_info {
                                    let _ = Self::commit_in_dir(&wt.path, &story);
                                }

                                // Merge worktree back to main
                                if let (Some(wt), Some(mgr)) = (&worktree_info, &worktree_mgr) {
                                    match mgr.merge(wt) {
                                        Ok(merge_result) => {
                                            if merge_result.success {
                                                info!(
                                                    story_id = %story.id,
                                                    files_changed = merge_result.files_changed,
                                                    "Merged story changes successfully"
                                                );
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
                                                // Cleanup worktree
                                                let _ = mgr.cleanup(wt);
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
                                                                .complete_merge(wt, &commit_msg)
                                                            {
                                                                Ok(final_result) => {
                                                                    if final_result.success {
                                                                        info!(
                                                                            story_id = %story.id,
                                                                            "Merge completed after conflict resolution"
                                                                        );
                                                                        self.state
                                                                            .prd
                                                                            .mark_passed(&story.id);
                                                                    } else {
                                                                        warn!(
                                                                            story_id = %story.id,
                                                                            "Merge failed even after resolution"
                                                                        );
                                                                        let _ = mgr.abort_merge();
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    warn!(
                                                                        story_id = %story.id,
                                                                        error = %e,
                                                                        "Failed to complete merge after resolution"
                                                                    );
                                                                    let _ = mgr.abort_merge();
                                                                }
                                                            }
                                                        } else {
                                                            warn!(
                                                                story_id = %story.id,
                                                                "Conflict resolver could not resolve all conflicts"
                                                            );
                                                            let _ = mgr.abort_merge();
                                                        }
                                                    }
                                                    Err(e) => {
                                                        warn!(
                                                            story_id = %story.id,
                                                            error = %e,
                                                            "Conflict resolver failed"
                                                        );
                                                        let _ = mgr.abort_merge();
                                                    }
                                                }
                                                // Cleanup worktree
                                                let _ = mgr.cleanup(wt);
                                            } else if merge_result.aborted {
                                                // Non-conflict failure that was already aborted
                                                warn!(
                                                    story_id = %story.id,
                                                    summary = %merge_result.summary,
                                                    "Merge was aborted due to non-conflict failure"
                                                );
                                                // Cleanup worktree
                                                let _ = mgr.cleanup(wt);
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
                                    // No worktree, just mark passed
                                    self.state.prd.mark_passed(&story.id);
                                    self.try_send_event(RalphEvent::StoryComplete {
                                        story_id: story.id.clone(),
                                        passed: true,
                                    });
                                }
                            } else {
                                warn!("Story {} failed quality checks", story.id);
                                self.try_send_event(RalphEvent::StoryComplete {
                                    story_id: story.id.clone(),
                                    passed: false,
                                });
                                // Cleanup worktree without merging
                                if let (Some(wt), Some(mgr)) = (&worktree_info, &worktree_mgr) {
                                    let _ = mgr.cleanup(wt);
                                }
                            }
                        } else {
                            // Failed - cleanup worktree without merging (keep for debugging)
                            self.try_send_event(RalphEvent::StoryError {
                                story_id: story.id.clone(),
                                error: "LLM call failed".to_string(),
                            });
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
        }

        Ok(())
    }

    /// Build prompt for a story (static version for parallel execution)
    fn build_story_prompt(
        story: &UserStory,
        prd_info: &(String, String),
        working_dir: &PathBuf,
    ) -> String {
        format!(
            r#"# PRD: {} - {}

## Working Directory: {}

## Current Story: {} - {}

{}

### Acceptance Criteria:
{}

## WORKFLOW (follow this exactly):

1. **EXPLORE** (2-4 tool calls): Use `glob` and `read` to understand existing code
2. **IMPLEMENT** (5-15 tool calls): Use `write` or `edit` to make changes
3. **VERIFY**: Run `bash` with command `cargo check 2>&1` to check for errors
4. **FIX OR FINISH**:
   - If no errors: Output `STORY_COMPLETE: {}` and STOP
   - If errors: Parse the error, fix it, re-run cargo check (max 3 fix attempts)
   - After 3 failed attempts: Output `STORY_BLOCKED: <error summary>` and STOP

## UNDERSTANDING CARGO ERRORS:

When `cargo check` fails, the output shows:
```
error[E0432]: unresolved import `crate::foo::bar`
  --> src/file.rs:10:5
   |
10 | use crate::foo::bar;
   |             ^^^ could not find `bar` in `foo`
```

Key parts:
- `error[E0432]` = error code (search rustc --explain E0432 for details)
- `src/file.rs:10:5` = file:line:column where error occurs
- The message explains what's wrong

COMMON FIXES:
- "unresolved import" → module doesn't exist or isn't exported, check mod.rs
- "cannot find" → typo in name or missing import
- "mismatched types" → wrong type, check function signatures
- "trait bound not satisfied" → missing impl or use statement

## TOOL USAGE:
- `read`: Read file content (always read before editing!)
- `edit`: Modify files (MUST include 3+ lines before/after for unique context)
- `write`: Create new files
- `bash`: Run commands with `{{"command": "...", "cwd": "{}"}}`

## CRITICAL RULES:
- ALWAYS read a file before editing it
- When edit fails with "ambiguous match", include MORE context lines
- Do NOT add TODO/placeholder comments
- Run `cargo check 2>&1` to see ALL errors including warnings
- Count your fix attempts - STOP after 3 failures

## TERMINATION:
SUCCESS: Output `STORY_COMPLETE: {}`
BLOCKED: Output `STORY_BLOCKED: <brief error description>`

Do NOT keep iterating indefinitely. Stop when done or blocked.
"#,
            prd_info.0,
            prd_info.1,
            working_dir.display(),
            story.id,
            story.title,
            story.description,
            story
                .acceptance_criteria
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            story.id,
            working_dir.display(),
            story.id
        )
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
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
            Arc::clone(provider),
            model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry, // Already an Arc<ToolRegistry>
            30,            // max steps per story (focused implementation)
            180,           // 3 minute timeout per story
            event_tx,
            story_id,
            bus.clone(),
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
        let wd = working_dir.display();
        let mut profiles = vec![
            (
                "auto-planner".to_string(),
                format!(
                    "You are @auto-planner.\n\
                     Specialty: Code analysis and implementation planning.\n\
                     Mission: Read the codebase to understand existing patterns, then produce a concrete step-by-step implementation plan.\n\
                     Working directory: {wd}\n\n\
                     Read existing code first using the `read` tool. Identify all files that need changes.\n\
                     Produce a numbered list of specific changes with file paths and code snippets.\n\
                     Pass your plan in a clear format the next agent can follow step by step."
                ),
                vec!["planning".into(), "analysis".into(), "relay".into()],
            ),
            (
                "auto-coder".to_string(),
                format!(
                    "You are @auto-coder.\n\
                     Specialty: Code implementation.\n\
                     Mission: Implement changes according to the plan, write production code, verify it compiles.\n\
                     Working directory: {wd}\n\n\
                     Follow the plan from the incoming handoff. Use `edit` and `write` tools to make changes.\n\
                     After implementing, run `bash` with `cargo check 2>&1` (cwd: {wd}) to verify.\n\
                     Fix any compilation errors. Report what you implemented."
                ),
                vec!["implementation".into(), "coding".into(), "relay".into()],
            ),
            (
                "auto-reviewer".to_string(),
                format!(
                    "You are @auto-reviewer.\n\
                     Specialty: Code review and quality verification.\n\
                     Mission: Review implemented changes, run quality checks, fix remaining issues.\n\
                     Working directory: {wd}\n\n\
                     Review the code changes from the incoming handoff. Run `bash` with `cargo check 2>&1` (cwd: {wd}).\n\
                     Fix any remaining issues. Verify the implementation meets the acceptance criteria.\n\
                     If complete, include STORY_COMPLETE in your output. If blocked, include STORY_BLOCKED."
                ),
                vec!["review".into(), "verification".into(), "relay".into()],
            ),
        ];

        if max_agents >= 4 {
            profiles.push((
                "auto-tester".to_string(),
                format!(
                    "You are @auto-tester.\n\
                     Specialty: Test writing and verification.\n\
                     Mission: Write tests for the implemented changes and ensure they pass.\n\
                     Working directory: {wd}\n\n\
                     Review the implementation, write appropriate tests, run them with `bash`.\n\
                     Report test results and coverage gaps."
                ),
                vec!["testing".into(), "relay".into()],
            ));
        }

        if max_agents >= 5 {
            profiles.push((
                "auto-refactorer".to_string(),
                format!(
                    "You are @auto-refactorer.\n\
                     Specialty: Code quality and lint compliance.\n\
                     Mission: Run clippy, fix warnings, improve code quality without changing behavior.\n\
                     Working directory: {wd}\n\n\
                     Run `bash` with `cargo clippy --all-features 2>&1` (cwd: {wd}).\n\
                     Fix warnings and improve naming, structure, error handling."
                ),
                vec!["quality".into(), "relay".into()],
            ));
        }

        profiles.truncate(max_agents);
        profiles
    }

    /// Normalize text for convergence detection in relay loops.
    fn normalize_for_relay(text: &str) -> String {
        let mut normalized = String::with_capacity(text.len().min(512));
        let mut last_was_space = false;
        for ch in text.chars() {
            if ch.is_ascii_alphanumeric() {
                normalized.push(ch.to_ascii_lowercase());
                last_was_space = false;
            } else if ch.is_whitespace() && !last_was_space {
                normalized.push(' ');
                last_was_space = true;
            }
            if normalized.len() >= 280 {
                break;
            }
        }
        normalized.trim().to_string()
    }

    /// Format relay handoff text between agents.
    fn prepare_relay_handoff(from_agent: &str, output: &str, original_task: &str) -> String {
        let max_len = 6_000;
        let relay_payload = if output.len() > max_len {
            let truncated: String = output.chars().take(max_len).collect();
            format!("{truncated}\n... (truncated)")
        } else {
            output.to_string()
        };

        format!(
            "Original task:\n{original_task}\n\n\
             Incoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide concrete next steps."
        )
    }

    /// Execute a story using a relay team of agents.
    ///
    /// Creates a team of specialist agents (planner, coder, reviewer, etc.),
    /// each with its own Session and full tool access, and runs them in a
    /// baton-passing relay loop. Each agent operates autonomously within its
    /// turn, using tools to read/write code and run commands.
    async fn call_relay_static(
        registry: &Arc<ProviderRegistry>,
        model: &str,
        prompt: &str,
        working_dir: &PathBuf,
        ralph_tx: Option<mpsc::Sender<RalphEvent>>,
        story_id: String,
        bus: Option<Arc<AgentBus>>,
        max_agents: usize,
        max_rounds: usize,
    ) -> anyhow::Result<String> {
        let max_agents = max_agents.clamp(2, 8);
        let max_rounds = max_rounds.clamp(1, 5);

        let profiles = Self::build_implementation_relay_profiles(max_agents, working_dir);

        // Create relay runtime (use existing bus or create a temporary one)
        let relay_bus = bus.unwrap_or_else(|| Arc::new(AgentBus::new()));
        let relay = ProtocolRelayRuntime::new(relay_bus.clone());

        let mut sessions: HashMap<String, Session> = HashMap::new();
        let mut ordered_agents: Vec<String> = Vec::new();
        let mut relay_profiles: Vec<RelayAgentProfile> = Vec::new();

        for (name, instructions, capabilities) in &profiles {
            let mut session = Session::new().await?;
            session.metadata.model = Some(model.to_string());
            session.agent = name.clone();
            session.bus = Some(relay_bus.clone());
            session.add_message(Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: instructions.clone(),
                }],
            });

            relay_profiles.push(RelayAgentProfile {
                name: name.clone(),
                capabilities: capabilities.clone(),
            });
            ordered_agents.push(name.clone());
            sessions.insert(name.clone(), session);
        }

        relay.register_agents(&relay_profiles);

        info!(
            story_id = %story_id,
            agents = ordered_agents.len(),
            rounds = max_rounds,
            "Starting relay team for story"
        );

        let mut baton = prompt.to_string();
        let mut previous_normalized: Option<String> = None;
        let mut convergence_hits = 0usize;
        let mut turns = 0usize;

        'relay_loop: for round in 1..=max_rounds {
            for idx in 0..ordered_agents.len() {
                let to = ordered_agents[idx].clone();
                let from = if idx == 0 {
                    if round == 1 {
                        "user".to_string()
                    } else {
                        ordered_agents[ordered_agents.len() - 1].clone()
                    }
                } else {
                    ordered_agents[idx - 1].clone()
                };

                turns += 1;
                relay.send_handoff(&from, &to, &baton);

                if let Some(ref tx) = ralph_tx {
                    let _ = tx
                        .send(RalphEvent::StoryToolCall {
                            story_id: story_id.clone(),
                            tool_name: format!(
                                "relay: @{from} → @{to} (round {round}/{max_rounds})"
                            ),
                        })
                        .await;
                }

                let Some(mut session) = sessions.remove(&to) else {
                    anyhow::bail!(
                        "Relay agent @{to} session unavailable for story {story_id}"
                    );
                };

                let (event_tx, mut event_rx) = mpsc::channel::<SessionEvent>(256);
                let registry_clone = registry.clone();
                let baton_clone = baton.clone();

                let join = tokio::spawn(async move {
                    let result = session
                        .prompt_with_events(&baton_clone, event_tx, registry_clone)
                        .await;
                    (session, result)
                });

                // Forward tool call events to Ralph UI while agent works
                while !join.is_finished() {
                    while let Ok(event) = event_rx.try_recv() {
                        if let Some(ref tx) = ralph_tx {
                            if let SessionEvent::ToolCallStart { ref name, .. } = event {
                                let _ = tx
                                    .send(RalphEvent::StoryToolCall {
                                        story_id: story_id.clone(),
                                        tool_name: format!("@{to}: {name}"),
                                    })
                                    .await;
                            }
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }

                let (updated_session, result) = join
                    .await
                    .map_err(|e| anyhow::anyhow!("Relay agent @{to} task error: {e}"))?;

                // Drain remaining events
                while event_rx.try_recv().is_ok() {}

                sessions.insert(to.clone(), updated_session);

                let output = result
                    .map_err(|e| anyhow::anyhow!("Relay agent @{to} failed on story {story_id}: {e}"))?
                    .text;

                // Convergence detection
                let normalized = Self::normalize_for_relay(&output);
                if previous_normalized.as_deref() == Some(normalized.as_str()) {
                    convergence_hits += 1;
                } else {
                    convergence_hits = 0;
                }
                previous_normalized = Some(normalized);

                if convergence_hits >= 2 {
                    info!(story_id = %story_id, turns, "Relay converged");
                    baton = output;
                    break 'relay_loop;
                }

                // Check if story reached terminal state
                if output.contains("STORY_COMPLETE") || output.contains("STORY_BLOCKED") {
                    info!(story_id = %story_id, turns, "Story reached terminal state via relay");
                    baton = output;
                    break 'relay_loop;
                }

                // Prepare handoff for next agent
                baton = Self::prepare_relay_handoff(&to, &output, prompt);
            }
        }

        relay.shutdown_agents(&ordered_agents);

        info!(
            story_id = %story_id,
            turns,
            convergence_hits,
            "Relay team completed"
        );

        Ok(baton)
    }

    /// Resolve merge conflicts using a dedicated sub-agent
    async fn resolve_conflicts_static(
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &PathBuf,
        story: &UserStory,
        conflicts: &[String],
        conflict_diffs: &[(String, String)],
        bus: Option<Arc<AgentBus>>,
    ) -> anyhow::Result<bool> {
        info!(
            story_id = %story.id,
            num_conflicts = conflicts.len(),
            "Starting conflict resolution sub-agent"
        );

        // Build prompt with conflict context
        let conflict_info = conflict_diffs
            .iter()
            .map(|(file, diff)| format!("### File: {}\n```diff\n{}\n```", file, diff))
            .collect::<Vec<_>>()
            .join("\n\n");

        let prompt = format!(
            r#"# CONFLICT RESOLUTION TASK

## Story Context: {} - {}
{}

## Conflicting Files
The following files have merge conflicts that need resolution:
{}

## Conflict Details
{}

## Your Task
1. Read each conflicting file to see the conflict markers
2. Understand what BOTH sides are trying to do:
   - HEAD (main branch): the current state
   - The incoming branch: the sub-agent's changes for story {}
3. Resolve each conflict by:
   - Keeping BOTH changes if they don't actually conflict
   - Merging the logic if they touch the same code
   - Preferring the sub-agent's changes if they implement the story requirement
4. Remove ALL conflict markers (<<<<<<<, =======, >>>>>>>)
5. Ensure the final code compiles: run `cargo check`

## CRITICAL RULES
- Do NOT leave any conflict markers in files
- Do NOT just pick one side - understand and merge the intent
- MUST run `cargo check` after resolving to verify
- Stage resolved files with `git add <file>`

## Termination
SUCCESS: Output `CONFLICTS_RESOLVED` when all files are resolved and compile
FAILED: Output `CONFLICTS_UNRESOLVED: <reason>` if you cannot resolve

Working directory: {}
"#,
            story.id,
            story.title,
            story.description,
            conflicts
                .iter()
                .map(|f| format!("- {}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            conflict_info,
            story.id,
            working_dir.display()
        );

        // Build system prompt
        let system_prompt = crate::agent::builtin::build_system_prompt(working_dir);

        // Create tool registry
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(provider), model.to_string());

        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(
            "Conflict resolver starting with {} tools",
            tool_definitions.len()
        );

        // Run the resolver with smaller limits (conflicts should be quick to resolve)
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
            Arc::clone(provider),
            model,
            &system_prompt,
            &prompt,
            tool_definitions,
            tool_registry,
            15,  // max 15 steps for conflict resolution
            120, // 2 min per-step timeout (resets on progress)
            None,
            String::new(),
            bus.clone(),
        )
        .await?;

        info!(
            story_id = %story.id,
            steps = steps,
            tool_calls = tool_calls,
            "Conflict resolver completed"
        );

        // Check if resolution was successful
        let resolved = output.contains("CONFLICTS_RESOLVED")
            || (output.contains("resolved") && !output.contains("UNRESOLVED"));

        if resolved {
            info!(story_id = %story.id, "Conflicts resolved successfully");
        } else {
            warn!(
                story_id = %story.id,
                output = %output.chars().take(200).collect::<String>(),
                "Conflict resolution may have failed"
            );
        }

        Ok(resolved)
    }

    /// Extract learnings (static version)
    fn extract_learnings_static(response: &str) -> Vec<String> {
        response
            .lines()
            .filter(|line| {
                line.contains("learned") || line.contains("Learning") || line.contains("# What")
            })
            .map(|line| line.trim().to_string())
            .collect()
    }

    /// Commit changes in a specific directory
    fn commit_in_dir(dir: &PathBuf, story: &UserStory) -> anyhow::Result<()> {
        // Stage all changes
        let _ = Command::new("git")
            .args(["add", "-A"])
            .current_dir(dir)
            .output();

        // Commit with story reference
        let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
        let _ = Command::new("git")
            .args(["commit", "-m", &msg])
            .current_dir(dir)
            .output();

        Ok(())
    }

    /// Find the directory containing Cargo.toml by walking down from the given directory.
    /// Returns the original directory if no Cargo.toml is found.
    ///
    /// This handles monorepo setups where the worktree root isn't the crate root.
    fn find_cargo_root(dir: &PathBuf) -> PathBuf {
        fn has_cargo_toml(path: &PathBuf) -> bool {
            path.join("Cargo.toml").exists()
        }

        // Check current directory first
        if has_cargo_toml(dir) {
            return dir.clone();
        }

        // Walk down into subdirectories (limited depth)
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut subdirs: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| e.path())
                .collect();

            // Sort for deterministic behavior
            subdirs.sort();

            for subdir in subdirs {
                if has_cargo_toml(&subdir) {
                    return subdir;
                }

                // Check one level deeper
                if let Ok(sub_entries) = std::fs::read_dir(&subdir) {
                    let mut sub_subdirs: Vec<_> = sub_entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .map(|e| e.path())
                        .collect();

                    sub_subdirs.sort();

                    for sub_subdir in sub_subdirs {
                        if has_cargo_toml(&sub_subdir) {
                            return sub_subdir;
                        }
                    }
                }
            }
        }

        // Fall back to original directory
        warn!(
            dir = %dir.display(),
            "No Cargo.toml found, using worktree root for cargo commands"
        );
        dir.clone()
    }

    /// Run quality gates in a specific directory with event emission
    async fn run_quality_gates_in_dir_with_events(
        &self,
        dir: &PathBuf,
        story_id: &str,
    ) -> anyhow::Result<bool> {
        // Find the Cargo root (where Cargo.toml lives)
        let cargo_root = Self::find_cargo_root(dir);
        debug!(
            worktree_dir = %dir.display(),
            cargo_root = %cargo_root.display(),
            "Running quality gates"
        );

        let checks = &self.state.prd.quality_checks;
        let mut all_passed = true;

        for (name, cmd) in [
            ("typecheck", &checks.typecheck),
            ("lint", &checks.lint),
            ("test", &checks.test),
            ("build", &checks.build),
        ] {
            if let Some(command) = cmd {
                debug!("Running {} check in {:?}: {}", name, cargo_root, command);
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(&cargo_root)
                    .output()
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to run quality check '{}': {}", name, e)
                    })?;

                let passed = output.status.success();
                self.try_send_event(RalphEvent::StoryQualityCheck {
                    story_id: story_id.to_string(),
                    check_name: name.to_string(),
                    passed,
                });

                if !passed {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let combined = format!("{}\n{}", stdout, stderr);
                    let error_summary: String = combined
                        .lines()
                        .filter(|line| {
                            line.starts_with("error")
                                || line.contains("error:")
                                || line.contains("error[")
                        })
                        .take(5)
                        .collect::<Vec<_>>()
                        .join("\n");
                    warn!(
                        check = %name,
                        cargo_root = %cargo_root.display(),
                        error_summary = %error_summary.chars().take(300).collect::<String>(),
                        "{} check failed in {:?}",
                        name,
                        cargo_root
                    );
                    all_passed = false;
                }
            }
        }

        Ok(all_passed)
    }

    /// Build the prompt for a story
    fn build_prompt(&self, story: &UserStory) -> String {
        let progress = self.load_progress().unwrap_or_default();

        format!(
            r#"# PRD: {} - {}

## Current Story: {} - {}

{}

### Acceptance Criteria:
{}

## Previous Progress:
{}

## Instructions:
1. Implement the requirements for this story
2. Write any necessary code changes
3. Document what you learned
4. End with `STORY_COMPLETE: {}` when done

Respond with the implementation and any shell commands needed.
"#,
            self.state.prd.project,
            self.state.prd.feature,
            story.id,
            story.title,
            story.description,
            story
                .acceptance_criteria
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            if progress.is_empty() {
                "None yet".to_string()
            } else {
                progress
            },
            story.id
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
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
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
        )
        .await?;

        info!(
            "Ralph agent completed: {} steps, {} tool calls",
            steps, tool_calls
        );

        Ok(output)
    }

    /// Run quality gates and emit events for each check
    async fn run_quality_gates_with_events(&self, story_id: &str) -> anyhow::Result<bool> {
        // Find the Cargo root (where Cargo.toml lives)
        let cargo_root = Self::find_cargo_root(&self.state.working_dir);
        debug!(
            working_dir = %self.state.working_dir.display(),
            cargo_root = %cargo_root.display(),
            "Running quality gates"
        );

        let checks = &self.state.prd.quality_checks;
        let mut all_passed = true;

        for (name, cmd) in [
            ("typecheck", &checks.typecheck),
            ("lint", &checks.lint),
            ("test", &checks.test),
            ("build", &checks.build),
        ] {
            if let Some(command) = cmd {
                debug!("Running {} check in {:?}: {}", name, cargo_root, command);
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(&cargo_root)
                    .output()
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to run quality check '{}': {}", name, e)
                    })?;

                let passed = output.status.success();
                self.try_send_event(RalphEvent::StoryQualityCheck {
                    story_id: story_id.to_string(),
                    check_name: name.to_string(),
                    passed,
                });

                if !passed {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let combined = format!("{}\n{}", stdout, stderr);
                    let error_summary: String = combined
                        .lines()
                        .filter(|line| {
                            line.starts_with("error")
                                || line.contains("error:")
                                || line.contains("error[")
                        })
                        .take(5)
                        .collect::<Vec<_>>()
                        .join("\n");
                    warn!(
                        check = %name,
                        cargo_root = %cargo_root.display(),
                        error_summary = %error_summary.chars().take(300).collect::<String>(),
                        "{} check failed in {:?}",
                        name, cargo_root
                    );
                    all_passed = false;
                    // Don't return early — run all checks so we can show all results
                }
            }
        }

        Ok(all_passed)
    }

    /// Commit changes for a story
    fn commit_story(&self, story: &UserStory) -> anyhow::Result<()> {
        info!("Committing changes for story: {}", story.id);

        // Stage all changes
        let _ = Command::new("git")
            .args(["add", "-A"])
            .current_dir(&self.state.working_dir)
            .output();

        // Commit with story reference
        let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
        match Command::new("git")
            .args(["commit", "-m", &msg])
            .current_dir(&self.state.working_dir)
            .output()
        {
            Ok(output) if output.status.success() => {
                info!("Committed: {}", msg);
            }
            Ok(output) => {
                warn!(
                    "Git commit had no changes or failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                warn!("Could not run git commit: {}", e);
            }
        }

        Ok(())
    }

    /// Git checkout
    fn git_checkout(&self, branch: &str) -> anyhow::Result<()> {
        // Try to checkout, create if doesn't exist
        let output = Command::new("git")
            .args(["checkout", branch])
            .current_dir(&self.state.working_dir)
            .output()?;

        if !output.status.success() {
            Command::new("git")
                .args(["checkout", "-b", branch])
                .current_dir(&self.state.working_dir)
                .output()?;
        }

        Ok(())
    }

    /// Load progress file
    fn load_progress(&self) -> anyhow::Result<String> {
        let path = self.state.working_dir.join(&self.config.progress_path);
        Ok(std::fs::read_to_string(path).unwrap_or_default())
    }

    /// Append to progress file
    fn append_progress(&self, entry: &ProgressEntry, response: &str) -> anyhow::Result<()> {
        let path = self.state.working_dir.join(&self.config.progress_path);
        let mut content = self.load_progress().unwrap_or_default();

        content.push_str(&format!(
            "\n---\n\n## Iteration {} - {} ({})\n\n**Status:** {}\n\n### Summary\n{}\n",
            entry.iteration, entry.story_id, entry.timestamp, entry.status, response
        ));

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Extract learnings from response
    fn extract_learnings(&self, response: &str) -> Vec<String> {
        let mut learnings = Vec::new();

        for line in response.lines() {
            if line.contains("learned") || line.contains("Learning") || line.contains("# What") {
                learnings.push(line.trim().to_string());
            }
        }

        learnings
    }

    /// Get current status
    pub fn status(&self) -> &RalphState {
        &self.state
    }

    /// Format status as markdown
    pub fn status_markdown(&self) -> String {
        let status = if self.state.prd.is_complete() {
            "# Ralph Complete!"
        } else {
            "# Ralph Status"
        };

        let stories: Vec<String> = self
            .state
            .prd
            .user_stories
            .iter()
            .map(|s| {
                let check = if s.passes { "[x]" } else { "[ ]" };
                format!("- {} {}: {}", check, s.id, s.title)
            })
            .collect();

        format!(
            "{}\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n**Iterations:** {}/{}\n\n## Stories\n{}",
            status,
            self.state.prd.project,
            self.state.prd.feature,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len(),
            self.state.current_iteration,
            self.state.max_iterations,
            stories.join("\n")
        )
    }
}

/// Create a sample PRD template
pub fn create_prd_template(project: &str, feature: &str) -> Prd {
    Prd {
        project: project.to_string(),
        feature: feature.to_string(),
        branch_name: format!("feature/{}", feature.to_lowercase().replace(' ', "-")),
        version: "1.0".to_string(),
        user_stories: vec![UserStory {
            id: "US-001".to_string(),
            title: "First user story".to_string(),
            description: "Description of what needs to be implemented".to_string(),
            acceptance_criteria: vec!["Criterion 1".to_string(), "Criterion 2".to_string()],
            passes: false,
            priority: 1,
            depends_on: Vec::new(),
            complexity: 3,
        }],
        technical_requirements: Vec::new(),
        quality_checks: QualityChecks {
            typecheck: Some("cargo check".to_string()),
            test: Some("cargo test".to_string()),
            lint: Some("cargo clippy".to_string()),
            build: Some("cargo build".to_string()),
        },
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}