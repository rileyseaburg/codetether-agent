//! Ralph loop - the core autonomous execution loop

use super::types::*;
use crate::provider::{CompletionRequest, Message, Provider, Role};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

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

        if self.state.status != RalphStatus::Completed {
            if self.state.current_iteration >= self.state.max_iterations {
                self.state.status = RalphStatus::MaxIterations;
            }
        }

        info!(
            "Ralph finished: {:?}, {}/{} stories passed",
            self.state.status,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len()
        );

        Ok(self.state.clone())
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
        
        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: prompt.to_string() }],
            }],
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
