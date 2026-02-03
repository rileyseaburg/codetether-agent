//! Ralph loop - the core autonomous execution loop

use super::types::*;
use crate::provider::{CompletionRequest, Message, Provider, Role};
use crate::worktree::WorktreeManager;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tracing::{info, warn, debug};

/// The main Ralph executor
pub struct RalphLoop {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
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
        })
    }

    /// Run the Ralph loop until completion or max iterations
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        self.state.status = RalphStatus::Running;

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
            && self.state.current_iteration >= self.state.max_iterations {
                self.state.status = RalphStatus::MaxIterations;
            }

        info!(
            "Ralph finished: {:?}, {}/{} stories passed",
            self.state.status,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len()
        );

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

            // Build the prompt
            let prompt = self.build_prompt(&story);

            // Call the LLM
            match self.call_llm(&prompt).await {
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
                    self.state.progress_log.push(entry);

                    // Run quality gates
                    if self.config.quality_checks_enabled {
                        if self.run_quality_gates().await? {
                            info!("Story {} passed quality checks!", story.id);
                            self.state.prd.mark_passed(&story.id);
                            
                            // Commit changes
                            if self.config.auto_commit {
                                self.commit_story(&story)?;
                            }
                            
                            // Save updated PRD
                            self.state.prd.save(&self.state.prd_path).await?;
                        } else {
                            warn!("Story {} failed quality checks", story.id);
                        }
                    } else {
                        // No quality checks, just mark as passed
                        self.state.prd.mark_passed(&story.id);
                        self.state.prd.save(&self.state.prd_path).await?;
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
                }
            }
        }
        
        Ok(())
    }
    
    /// Run stories in parallel by stage with worktree isolation
    async fn run_parallel(&mut self) -> anyhow::Result<()> {
        // Clone stages upfront to avoid borrow issues
        let stages: Vec<Vec<UserStory>> = self.state.prd.stages()
            .into_iter()
            .map(|stage| stage.into_iter().cloned().collect())
            .collect();
        let total_stages = stages.len();
        
        info!(
            "Parallel execution: {} stages, {} max concurrent stories",
            total_stages,
            self.config.max_concurrent_stories
        );
        
        // Create worktree manager if enabled
        let worktree_mgr = if self.config.worktree_enabled {
            match WorktreeManager::new(&self.state.working_dir) {
                Ok(mgr) => {
                    info!("Worktree isolation enabled for parallel stories");
                    Some(Arc::new(mgr))
                }
                Err(e) => {
                    warn!("Failed to create worktree manager: {}, falling back to sequential within stages", e);
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
            let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.max_concurrent_stories));
            let provider = Arc::clone(&self.provider);
            let model = self.model.clone();
            let prd_info = (self.state.prd.project.clone(), self.state.prd.feature.clone());
            let working_dir = self.state.working_dir.clone();
            let progress_path = self.config.progress_path.clone();
            
            let mut handles = Vec::new();
            
            for story in stories {
                let sem = Arc::clone(&semaphore);
                let provider = Arc::clone(&provider);
                let model = model.clone();
                let prd_info = prd_info.clone();
                let working_dir = working_dir.clone();
                let worktree_mgr = worktree_mgr.clone();
                let progress_path = progress_path.clone();
                
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
                    
                    info!("Working on story: {} - {} (in {:?})", story.id, story.title, story_working_dir);
                    
                    // Build the prompt with worktree awareness
                    let prompt = Self::build_story_prompt(&story, &prd_info, &story_working_dir);
                    
                    // Call the LLM
                    let result = Self::call_llm_static(&provider, &model, &prompt, &story_working_dir).await;
                    
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
                            let check_dir = worktree_info.as_ref()
                                .map(|wt| wt.path.clone())
                                .unwrap_or_else(|| self.state.working_dir.clone());
                            
                            let quality_passed = if self.config.quality_checks_enabled {
                                self.run_quality_gates_in_dir(&check_dir).await.unwrap_or(false)
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
                                            } else {
                                                warn!(
                                                    story_id = %story.id,
                                                    conflicts = ?merge_result.conflicts,
                                                    "Merge had conflicts"
                                                );
                                            }
                                            
                                            // Cleanup worktree
                                            let _ = mgr.cleanup(wt);
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
                                }
                            } else {
                                warn!("Story {} failed quality checks", story.id);
                                // Cleanup worktree without merging
                                if let (Some(wt), Some(mgr)) = (&worktree_info, &worktree_mgr) {
                                    let _ = mgr.cleanup(wt);
                                }
                            }
                        } else {
                            // Failed - cleanup worktree without merging (keep for debugging)
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
            
            // Save PRD after each stage
            self.state.prd.save(&self.state.prd_path).await?;
        }
        
        Ok(())
    }
    
    /// Build prompt for a story (static version for parallel execution)
    fn build_story_prompt(story: &UserStory, prd_info: &(String, String), working_dir: &PathBuf) -> String {
        format!(
            r#"# PRD: {} - {}

## Working Directory: {}

## Current Story: {} - {}

{}

### Acceptance Criteria:
{}

## Instructions:
1. Implement the requirements for this story
2. Write any necessary code changes using the working directory above
3. Document what you learned
4. End with `STORY_COMPLETE: {}` when done

Respond with the implementation and any shell commands needed.
"#,
            prd_info.0,
            prd_info.1,
            working_dir.display(),
            story.id,
            story.title,
            story.description,
            story.acceptance_criteria.iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            story.id
        )
    }
    
    /// Call LLM (static version for parallel execution)
    async fn call_llm_static(provider: &Arc<dyn Provider>, model: &str, prompt: &str, working_dir: &PathBuf) -> anyhow::Result<String> {
        use crate::provider::ContentPart;
        
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(working_dir);
        
        let request = CompletionRequest {
            messages: vec![
                Message {
                    role: Role::System,
                    content: vec![ContentPart::Text { text: system_prompt }],
                },
                Message {
                    role: Role::User,
                    content: vec![ContentPart::Text { text: prompt.to_string() }],
                },
            ],
            tools: Vec::new(),
            model: model.to_string(),
            temperature: Some(0.7),
            top_p: None,
            max_tokens: Some(4096),
            stop: Vec::new(),
        };

        let result = timeout(
            Duration::from_secs(300), // Longer timeout for parallel
            provider.complete(request)
        ).await;

        match result {
            Ok(Ok(response)) => {
                let text = response.message.content.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                Ok(text)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!("LLM call timed out after 300 seconds")),
        }
    }
    
    /// Extract learnings (static version)
    fn extract_learnings_static(response: &str) -> Vec<String> {
        response.lines()
            .filter(|line| line.contains("learned") || line.contains("Learning") || line.contains("# What"))
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
    
    /// Run quality gates in a specific directory
    async fn run_quality_gates_in_dir(&self, dir: &PathBuf) -> anyhow::Result<bool> {
        let checks = &self.state.prd.quality_checks;
        
        for (name, cmd) in [
            ("typecheck", &checks.typecheck),
            ("lint", &checks.lint),
            ("test", &checks.test),
            ("build", &checks.build),
        ] {
            if let Some(command) = cmd {
                debug!("Running {} check in {:?}: {}", name, dir, command);
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(dir)
                    .output()
                    .map_err(|e| anyhow::anyhow!("Failed to run quality check '{}': {}", name, e))?;
                
                if !output.status.success() {
                    warn!("{} check failed in {:?}: {}", name, dir,
                        String::from_utf8_lossy(&output.stderr));
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
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
            story.acceptance_criteria.iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            if progress.is_empty() { "None yet".to_string() } else { progress },
            story.id
        )
    }

    /// Call the LLM with a prompt
    async fn call_llm(&self, prompt: &str) -> anyhow::Result<String> {
        use crate::provider::ContentPart;
        
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(&self.state.working_dir);
        
        let request = CompletionRequest {
            messages: vec![
                Message {
                    role: Role::System,
                    content: vec![ContentPart::Text { text: system_prompt }],
                },
                Message {
                    role: Role::User,
                    content: vec![ContentPart::Text { text: prompt.to_string() }],
                },
            ],
            tools: Vec::new(),
            model: self.model.clone(),
            temperature: Some(0.7),
            top_p: None,
            max_tokens: Some(4096),
            stop: Vec::new(),
        };

        let result = timeout(
            Duration::from_secs(120),
            self.provider.complete(request)
        ).await;

        match result {
            Ok(Ok(response)) => {
                // Extract text from response message
                let text = response.message.content.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                Ok(text)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!("LLM call timed out after 120 seconds")),
        }
    }

    /// Run quality gates
    async fn run_quality_gates(&self) -> anyhow::Result<bool> {
        let checks = &self.state.prd.quality_checks;
        
        for (name, cmd) in [
            ("typecheck", &checks.typecheck),
            ("lint", &checks.lint),
            ("test", &checks.test),
            ("build", &checks.build),
        ] {
            if let Some(command) = cmd {
                info!("Running {} check in {:?}: {}", name, self.state.working_dir, command);
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(&self.state.working_dir)
                    .output()
                    .map_err(|e| anyhow::anyhow!("Failed to run quality check '{}': {}", name, e))?;
                
                if !output.status.success() {
                    warn!("{} check failed: {}", name, 
                        String::from_utf8_lossy(&output.stderr));
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
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
                warn!("Git commit had no changes or failed: {}",
                    String::from_utf8_lossy(&output.stderr));
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
            entry.iteration,
            entry.story_id,
            entry.timestamp,
            entry.status,
            response
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

        let stories: Vec<String> = self.state.prd.user_stories.iter()
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
        user_stories: vec![
            UserStory {
                id: "US-001".to_string(),
                title: "First user story".to_string(),
                description: "Description of what needs to be implemented".to_string(),
                acceptance_criteria: vec![
                    "Criterion 1".to_string(),
                    "Criterion 2".to_string(),
                ],
                passes: false,
                priority: 1,
                depends_on: Vec::new(),
                complexity: 3,
            },
        ],
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
