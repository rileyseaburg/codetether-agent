//! Benchmark runner - orchestrates benchmark execution across models and PRDs
//!
//! For each benchmark PRD × model combination, the runner:
//! 1. Scaffolds a fresh Rust project in a temp directory (`cargo init`)
//! 2. Initializes git so Ralph can create branches and commit
//! 3. Copies the benchmark PRD in as `prd.json`
//! 4. Runs Ralph's full autonomous loop (swarm agent, quality gates, etc.)
//! 5. Records pass/fail per story, token usage, duration, and cost
//! 6. Fetches real pricing from models.dev for accurate cost reporting
//! 7. Optionally submits results to the benchmark API

use super::types::*;
use crate::provider::ProviderRegistry;
use crate::ralph::{Prd, QualityChecks, RalphConfig, RalphLoop};
use crate::telemetry::TOKEN_USAGE;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use tracing::{error, info, warn};

/// Pricing from models.dev for a single model
#[derive(Debug, Clone)]
struct ModelPricing {
    /// Cost per million input tokens (USD)
    input_per_m: f64,
    /// Cost per million output tokens (USD)
    output_per_m: f64,
}

/// Main benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Discover benchmark PRD files in the configured directory
    fn discover_prds(&self) -> anyhow::Result<Vec<(PathBuf, u8)>> {
        let dir = Path::new(&self.config.prd_dir);
        if !dir.exists() {
            anyhow::bail!("Benchmark directory not found: {}", self.config.prd_dir);
        }

        let mut prds = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let tier = detect_tier(&filename);

                // Filter by tier if specified
                if let Some(filter_tier) = self.config.tier
                    && tier != filter_tier
                {
                    continue;
                }

                prds.push((path, tier));
            }
        }

        prds.sort_by_key(|(_, tier)| *tier);
        Ok(prds)
    }

    /// Parse model string into (provider, model) tuple
    fn parse_model(model_str: &str) -> anyhow::Result<(String, String)> {
        let parts: Vec<&str> = model_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            anyhow::bail!(
                "Invalid model format '{}'. Expected 'provider:model'",
                model_str
            );
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// Scaffold a fresh Rust workspace in a temporary directory for benchmark isolation.
    ///
    /// Returns (temp_dir_handle, path to prd.json inside the workspace).
    /// The temp dir is automatically deleted when the handle is dropped.
    fn scaffold_workspace(
        prd_path: &Path,
        prd: &Prd,
    ) -> anyhow::Result<(tempfile::TempDir, PathBuf)> {
        let tmp = tempfile::Builder::new()
            .prefix(&format!("bench-{}-", prd.project))
            .tempdir()?;
        let workspace = tmp.path();

        info!(
            "Scaffolding benchmark workspace at {:?} for PRD '{}'",
            workspace, prd.feature
        );

        // cargo init --name <project>
        let cargo_init = Command::new("cargo")
            .args(["init", "--name", &prd.project])
            .current_dir(workspace)
            .output()?;
        if !cargo_init.status.success() {
            anyhow::bail!(
                "cargo init failed: {}",
                String::from_utf8_lossy(&cargo_init.stderr)
            );
        }

        // Add common benchmark dependencies to Cargo.toml
        // which crates to add depends on the PRD
        let mut cargo_add_args: Vec<&str> = vec!["add", "serde", "--features", "derive"];
        let cargo_add = Command::new("cargo")
            .args(&cargo_add_args)
            .current_dir(workspace)
            .output()?;
        if !cargo_add.status.success() {
            warn!(
                "cargo add serde failed (non-fatal): {}",
                String::from_utf8_lossy(&cargo_add.stderr)
            );
        }

        // Add dependencies based on PRD requirements
        let feature_lower = prd.feature.to_lowercase();
        let desc_text: String = prd
            .user_stories
            .iter()
            .map(|s| format!("{} {}", s.title, s.description))
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        if feature_lower.contains("api")
            || feature_lower.contains("rest")
            || desc_text.contains("axum")
            || desc_text.contains("endpoint")
        {
            cargo_add_args = vec!["add", "axum", "tokio", "--features", "tokio/full"];
            let _ = Command::new("cargo")
                .args(&cargo_add_args)
                .current_dir(workspace)
                .output();
            // Also add serde_json for JSON responses
            let _ = Command::new("cargo")
                .args(["add", "serde_json"])
                .current_dir(workspace)
                .output();
        }

        if desc_text.contains("clap") || feature_lower.contains("cli") {
            let _ = Command::new("cargo")
                .args(["add", "clap", "--features", "derive"])
                .current_dir(workspace)
                .output();
        }

        if desc_text.contains("csv") {
            let _ = Command::new("cargo")
                .args(["add", "csv"])
                .current_dir(workspace)
                .output();
        }

        // git init + initial commit so Ralph can create branches
        let git_init = Command::new("git")
            .args(["init"])
            .current_dir(workspace)
            .output()?;
        if !git_init.status.success() {
            anyhow::bail!(
                "git init failed: {}",
                String::from_utf8_lossy(&git_init.stderr)
            );
        }

        // Set git user for commits (required for CI environments)
        let _ = Command::new("git")
            .args(["config", "user.email", "bench@codetether.run"])
            .current_dir(workspace)
            .output();
        let _ = Command::new("git")
            .args(["config", "user.name", "CodeTether Benchmark"])
            .current_dir(workspace)
            .output();

        // Initial commit with scaffolded project
        let _ = Command::new("git")
            .args(["add", "-A"])
            .current_dir(workspace)
            .output();
        let _ = Command::new("git")
            .args(["commit", "-m", "initial scaffold for benchmark"])
            .current_dir(workspace)
            .output();

        // Copy the benchmark PRD into the workspace as prd.json
        let dest_prd = workspace.join("prd.json");
        std::fs::copy(prd_path, &dest_prd)?;

        Ok((tmp, dest_prd))
    }

    /// Run quality checks in a directory and collect results
    fn run_quality_checks(working_dir: &Path, checks: &QualityChecks) -> Vec<QualityCheckResult> {
        let mut results = Vec::new();

        for (name, cmd) in [
            ("typecheck", &checks.typecheck),
            ("lint", &checks.lint),
            ("test", &checks.test),
            ("build", &checks.build),
        ] {
            if let Some(command) = cmd {
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(working_dir)
                    .output();

                let (passed, output_text) = match output {
                    Ok(o) => {
                        let passed = o.status.success();
                        let text = if passed {
                            None
                        } else {
                            let stderr = String::from_utf8_lossy(&o.stderr);
                            let stdout = String::from_utf8_lossy(&o.stdout);
                            Some(format!("{}\n{}", stdout, stderr))
                        };
                        (passed, text)
                    }
                    Err(e) => (false, Some(format!("Failed to execute: {}", e))),
                };

                results.push(QualityCheckResult {
                    name: name.to_string(),
                    passed,
                    output: output_text,
                });
            }
        }

        results
    }

    /// Fetch model pricing from models.dev API
    async fn fetch_pricing(provider_id: &str, model_id: &str) -> Option<ModelPricing> {
        let url = "https://models.dev/api.json";
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .ok()?;

        let resp = client.get(url).send().await.ok()?;
        let data: serde_json::Value = resp.json().await.ok()?;

        // Navigate: providers[provider_id].models[model_id].cost
        let cost = data
            .get(provider_id)?
            .get("models")?
            .get(model_id)?
            .get("cost")?;

        let input = cost.get("input")?.as_f64()?;
        let output = cost.get("output")?.as_f64()?;

        Some(ModelPricing {
            input_per_m: input,
            output_per_m: output,
        })
    }

    /// Calculate cost from token counts and pricing
    fn calculate_cost(
        input_tokens: u64,
        output_tokens: u64,
        pricing: &Option<ModelPricing>,
    ) -> f64 {
        match pricing {
            Some(p) => {
                (input_tokens as f64 / 1_000_000.0) * p.input_per_m
                    + (output_tokens as f64 / 1_000_000.0) * p.output_per_m
            }
            // Fallback: ~$5/1M tokens average when pricing unavailable
            None => (input_tokens + output_tokens) as f64 * 0.000005,
        }
    }

    /// Submit results to the benchmark API
    async fn submit_results(result: &BenchmarkSuiteResult, api_url: &str, api_key: &str) {
        let client = match reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to create HTTP client for submission: {}", e);
                return;
            }
        };

        let result_json = match serde_json::to_string(result) {
            Ok(j) => j,
            Err(e) => {
                warn!("Failed to serialize results: {}", e);
                return;
            }
        };

        for mr in &result.model_results {
            let submission = crate::benchmark::types::BenchmarkSubmission {
                model: mr.model.clone(),
                agent: format!("{} v{}", result.agent, result.agent_version),
                result: result_json.clone(),
            };

            match client
                .post(api_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&submission)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    info!("Submitted benchmark results for {}", mr.model);
                }
                Ok(resp) => {
                    warn!(
                        "Benchmark submission failed for {} (HTTP {})",
                        mr.model,
                        resp.status()
                    );
                }
                Err(e) => {
                    warn!("Failed to submit benchmark results for {}: {}", mr.model, e);
                }
            }
        }
    }

    /// Run the full benchmark suite
    pub async fn run(&self) -> anyhow::Result<BenchmarkSuiteResult> {
        let start = Instant::now();
        let prds = self.discover_prds()?;

        if prds.is_empty() {
            anyhow::bail!("No benchmark PRDs found in {}", self.config.prd_dir);
        }

        info!("Discovered {} benchmark PRDs across tiers", prds.len());

        let mut model_results = Vec::new();

        for model_str in &self.config.models {
            let (provider_name, model_name) = Self::parse_model(model_str)?;
            info!("Benchmarking model: {}:{}", provider_name, model_name);

            // Load provider from Vault
            let registry = ProviderRegistry::from_vault().await?;
            let provider = registry.get(&provider_name).ok_or_else(|| {
                anyhow::anyhow!("Provider '{}' not found in Vault", provider_name)
            })?;

            // Fetch pricing once per model
            let pricing = Self::fetch_pricing(&provider_name, &model_name).await;
            if let Some(ref p) = pricing {
                info!(
                    "Model pricing: ${:.2}/M input, ${:.2}/M output",
                    p.input_per_m, p.output_per_m
                );
            } else {
                warn!(
                    "Could not fetch pricing for {}:{}, using fallback estimates",
                    provider_name, model_name
                );
            }

            let mut prd_results = Vec::new();
            let mut total_cost = 0.0_f64;

            for (prd_path, tier) in &prds {
                // Check cost ceiling before starting
                if let Some(ceiling) = self.config.cost_ceiling_usd
                    && total_cost >= ceiling
                {
                    warn!(
                        "Cost ceiling ${:.2} reached — skipping remaining PRDs for model {}",
                        ceiling, model_str
                    );
                    break;
                }

                let prd_id = prd_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                info!("Running benchmark PRD: {} (tier {})", prd_id, tier);

                match self
                    .run_single_prd(prd_path, *tier, provider.clone(), &model_name, &pricing)
                    .await
                {
                    Ok(result) => {
                        total_cost += result.cost_usd;
                        info!(
                            "PRD {} complete: {}/{} stories passed ({:.0}%) — ${:.4}",
                            prd_id,
                            result.stories_passed,
                            result.stories_total,
                            result.pass_rate * 100.0,
                            result.cost_usd,
                        );
                        prd_results.push(result);
                    }
                    Err(e) => {
                        error!("Failed to run benchmark PRD {}: {}", prd_id, e);
                        prd_results.push(PrdBenchmarkResult {
                            prd_id,
                            prd_tier: *tier,
                            prd_feature: "FAILED".to_string(),
                            stories_total: 0,
                            stories_passed: 0,
                            pass_rate: 0.0,
                            duration_seconds: 0.0,
                            tokens_used: 0,
                            cost_usd: 0.0,
                            quality_checks: Vec::new(),
                            per_story: Vec::new(),
                        });
                    }
                }
            }

            let aggregate = Self::compute_aggregate(&prd_results);
            model_results.push(ModelBenchmarkResult {
                model: model_str.clone(),
                prd_results,
                aggregate,
            });
        }

        let summary = Self::compute_summary(&model_results);
        let elapsed = start.elapsed();

        let result = BenchmarkSuiteResult {
            run_date: chrono::Utc::now().to_rfc3339(),
            agent: "codetether".to_string(),
            agent_version: env!("CARGO_PKG_VERSION").to_string(),
            model_results,
            summary,
        };

        info!("Benchmark suite complete in {:.1}s", elapsed.as_secs_f64());

        // Write results to file
        let output_path = Path::new(&self.config.output);
        let json = serde_json::to_string_pretty(&result)?;
        tokio::fs::write(output_path, &json).await?;
        info!("Results written to {}", self.config.output);

        // Submit to API if configured
        if let (Some(api_url), Some(api_key)) =
            (&self.config.submit_api_url, &self.config.submit_api_key)
        {
            Self::submit_results(&result, api_url, api_key).await;
        }

        Ok(result)
    }

    /// Run a single PRD benchmark in an isolated workspace
    async fn run_single_prd(
        &self,
        prd_path: &Path,
        tier: u8,
        provider: std::sync::Arc<dyn crate::provider::Provider>,
        model: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<PrdBenchmarkResult> {
        let prd = Prd::load(&prd_path.to_path_buf()).await?;
        let prd_id = prd_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let prd_feature = prd.feature.clone();
        let prd_quality_checks = prd.quality_checks.clone();

        // Scaffold an isolated workspace (temp dir with cargo init + git init)
        let (_tmp_handle, workspace_prd_path) = Self::scaffold_workspace(prd_path, &prd)?;
        let workspace_dir: &Path = workspace_prd_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid workspace PRD path"))?;

        info!(
            "Benchmark workspace ready at {:?} for PRD '{}'",
            workspace_dir, prd_feature
        );

        // Snapshot token usage before
        let tokens_before = TOKEN_USAGE.global_snapshot();

        let start = Instant::now();

        // Configure Ralph to run in the benchmark workspace
        let config = RalphConfig {
            prd_path: workspace_prd_path.to_string_lossy().to_string(),
            max_iterations: self.config.max_iterations,
            story_timeout_secs: self.config.story_timeout_secs,
            quality_checks_enabled: true,
            auto_commit: true, // Commit per story for clean git history
            parallel_enabled: self.config.parallel,
            ..Default::default()
        };

        // RalphLoop::new reads the PRD and uses its parent as working_dir
        let mut ralph_loop = RalphLoop::new(
            workspace_prd_path.clone(),
            provider,
            model.to_string(),
            config,
        )
        .await?;

        let state = ralph_loop.run().await?;
        let duration = start.elapsed();

        // Snapshot token usage after
        let tokens_after = TOKEN_USAGE.global_snapshot();
        let input_tokens = tokens_after
            .total
            .input
            .saturating_sub(tokens_before.total.input);
        let output_tokens = tokens_after
            .total
            .output
            .saturating_sub(tokens_before.total.output);
        let tokens_used = input_tokens + output_tokens;

        // Calculate cost using real pricing
        let cost_usd = Self::calculate_cost(input_tokens, output_tokens, pricing);

        // Build per-story results from Ralph's state
        let per_story: Vec<StoryBenchmarkResult> = state
            .prd
            .user_stories
            .iter()
            .map(|story| {
                let progress_entries: Vec<_> = state
                    .progress_log
                    .iter()
                    .filter(|p| p.story_id == story.id)
                    .collect();

                StoryBenchmarkResult {
                    story_id: story.id.clone(),
                    title: story.title.clone(),
                    passed: story.passes,
                    iterations: progress_entries.len(),
                    duration_seconds: 0.0, // Per-story timing not tracked in Ralph
                    tokens_used: 0,        // Per-story tokens not tracked separately
                    files_changed: progress_entries
                        .iter()
                        .flat_map(|p| p.files_changed.iter().cloned())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect(),
                }
            })
            .collect();

        let stories_passed = state.prd.passed_count();
        let stories_total = state.prd.user_stories.len();

        // Run final quality checks to capture their state for reporting
        let quality_checks = Self::run_quality_checks(workspace_dir, &prd_quality_checks);

        Ok(PrdBenchmarkResult {
            prd_id,
            prd_tier: tier,
            prd_feature,
            stories_total,
            stories_passed,
            pass_rate: if stories_total > 0 {
                stories_passed as f64 / stories_total as f64
            } else {
                0.0
            },
            duration_seconds: duration.as_secs_f64(),
            tokens_used,
            cost_usd,
            quality_checks,
            per_story,
        })
    }

    /// Compute aggregate metrics from PRD results
    fn compute_aggregate(prd_results: &[PrdBenchmarkResult]) -> AggregateMetrics {
        let prds_attempted = prd_results.len();
        let prds_fully_passed = prd_results.iter().filter(|r| r.pass_rate >= 1.0).count();

        let total_stories: usize = prd_results.iter().map(|r| r.stories_total).sum();
        let total_passed: usize = prd_results.iter().map(|r| r.stories_passed).sum();
        let total_tokens: u64 = prd_results.iter().map(|r| r.tokens_used).sum();
        let total_cost: f64 = prd_results.iter().map(|r| r.cost_usd).sum();
        let total_duration: f64 = prd_results.iter().map(|r| r.duration_seconds).sum();

        let overall_pass_rate = if total_stories > 0 {
            total_passed as f64 / total_stories as f64
        } else {
            0.0
        };

        let avg_seconds_per_story = if total_passed > 0 {
            total_duration / total_passed as f64
        } else {
            0.0
        };

        let avg_tokens_per_story = if total_passed > 0 {
            total_tokens as f64 / total_passed as f64
        } else {
            0.0
        };

        let stories_per_hour = if total_duration > 0.0 {
            total_passed as f64 / (total_duration / 3600.0)
        } else {
            0.0
        };

        AggregateMetrics {
            prds_attempted,
            prds_fully_passed,
            overall_pass_rate,
            total_stories,
            total_stories_passed: total_passed,
            avg_seconds_per_story,
            avg_tokens_per_story,
            total_cost_usd: total_cost,
            avg_cost_per_story: if total_passed > 0 {
                total_cost / total_passed as f64
            } else {
                0.0
            },
            total_duration_seconds: total_duration,
            stories_per_hour,
        }
    }

    /// Compute summary rankings across all models
    fn compute_summary(model_results: &[ModelBenchmarkResult]) -> BenchmarkSummary {
        if model_results.is_empty() {
            return BenchmarkSummary {
                best_pass_rate_model: String::new(),
                fastest_model: String::new(),
                cheapest_model: String::new(),
                best_overall_model: String::new(),
                rankings: Vec::new(),
            };
        }

        // Find max values for normalization
        let max_pass_rate = model_results
            .iter()
            .map(|r| r.aggregate.overall_pass_rate)
            .fold(0.0_f64, f64::max);
        let max_speed = model_results
            .iter()
            .map(|r| r.aggregate.stories_per_hour)
            .fold(0.0_f64, f64::max);
        let min_cost = model_results
            .iter()
            .filter(|r| r.aggregate.avg_cost_per_story > 0.0)
            .map(|r| r.aggregate.avg_cost_per_story)
            .fold(f64::INFINITY, f64::min);

        let mut rankings: Vec<ModelRanking> = model_results
            .iter()
            .map(|r| {
                let pass_rate_score = if max_pass_rate > 0.0 {
                    (r.aggregate.overall_pass_rate / max_pass_rate) * 100.0
                } else {
                    0.0
                };

                let speed_score = if max_speed > 0.0 {
                    (r.aggregate.stories_per_hour / max_speed) * 100.0
                } else {
                    0.0
                };

                let cost_score = if r.aggregate.avg_cost_per_story > 0.0 && min_cost.is_finite() {
                    (min_cost / r.aggregate.avg_cost_per_story) * 100.0
                } else {
                    0.0
                };

                // Weighted: 50% accuracy, 25% speed, 25% cost
                let overall_score = pass_rate_score * 0.50 + speed_score * 0.25 + cost_score * 0.25;

                ModelRanking {
                    model: r.model.clone(),
                    pass_rate_score,
                    speed_score,
                    cost_score,
                    overall_score,
                }
            })
            .collect();

        rankings.sort_by(|a, b| {
            b.overall_score
                .partial_cmp(&a.overall_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_pass = model_results
            .iter()
            .max_by(|a, b| {
                a.aggregate
                    .overall_pass_rate
                    .partial_cmp(&b.aggregate.overall_pass_rate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.model.clone())
            .unwrap_or_default();

        let fastest = model_results
            .iter()
            .max_by(|a, b| {
                a.aggregate
                    .stories_per_hour
                    .partial_cmp(&b.aggregate.stories_per_hour)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.model.clone())
            .unwrap_or_default();

        let cheapest = model_results
            .iter()
            .filter(|r| r.aggregate.avg_cost_per_story > 0.0)
            .min_by(|a, b| {
                a.aggregate
                    .avg_cost_per_story
                    .partial_cmp(&b.aggregate.avg_cost_per_story)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.model.clone())
            .unwrap_or_default();

        let best_overall = rankings
            .first()
            .map(|r| r.model.clone())
            .unwrap_or_default();

        BenchmarkSummary {
            best_pass_rate_model: best_pass,
            fastest_model: fastest,
            cheapest_model: cheapest,
            best_overall_model: best_overall,
            rankings,
        }
    }
}
