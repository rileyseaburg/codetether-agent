//! Deterministic collapse controller for speculative swarm branches.
//!
//! Phase 0 focuses on low-latency, mechanical regulation:
//! - sample active worktrees on an interval
//! - compute a coherence score vector per branch
//! - kill low-coherence branches deterministically
//! - promote one branch as the current integration candidate

use anyhow::{Context, Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Multi-dimensional coherence score for a branch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoherenceScore {
    pub compile_health: f32,
    pub test_health: f32,
    pub contract_alignment: f32,
    pub diff_conflict_risk: f32,
    pub velocity: f32,
    pub resource_health_score: f32,
    pub invariant_breaks: u32,
}

impl CoherenceScore {
    pub fn collapse_score(&self) -> f32 {
        // Penalize conflicts/invariant breaks and reward deterministic health signals.
        let weighted = (self.compile_health * 0.29)
            + (self.test_health * 0.14)
            + (self.contract_alignment * 0.18)
            + ((1.0 - self.diff_conflict_risk) * 0.14)
            + (self.velocity * 0.08)
            + (self.resource_health_score * 0.17);

        // Hard penalty once invariant breaks begin to accumulate.
        let invariant_penalty = (self.invariant_breaks as f32 * 0.1).clamp(0.0, 0.6);
        (weighted - invariant_penalty).clamp(0.0, 1.0)
    }
}

/// Runtime state for a currently active branch/worktree.
#[derive(Debug, Clone)]
pub struct BranchRuntimeState {
    pub subtask_id: String,
    pub branch: String,
    pub worktree_path: PathBuf,
}

/// Full evaluation output for one branch.
#[derive(Debug, Clone)]
pub struct BranchEvaluation {
    pub subtask_id: String,
    pub branch: String,
    pub score: CoherenceScore,
    pub aggregate_score: f32,
}

/// Branch observation produced by any execution backend.
#[derive(Debug, Clone)]
pub struct BranchObservation {
    pub subtask_id: String,
    pub branch: String,
    pub compile_ok: bool,
    pub changed_files: HashSet<String>,
    pub changed_lines: u32,
    pub resource_health_score: f32,
    pub infra_unhealthy_signals: u32,
}

/// Deterministic policy knobs for collapse behavior.
#[derive(Debug, Clone, Copy)]
pub struct CollapsePolicy {
    pub max_invariant_breaks: u32,
    pub max_consecutive_compile_failures: u32,
    pub max_consecutive_resource_failures: u32,
    pub min_collapse_score: f32,
    pub min_score_gap_for_prune: f32,
    pub min_resource_health_score: f32,
}

impl Default for CollapsePolicy {
    fn default() -> Self {
        Self {
            max_invariant_breaks: 2,
            max_consecutive_compile_failures: 1,
            max_consecutive_resource_failures: 2,
            min_collapse_score: 0.30,
            min_score_gap_for_prune: 0.15,
            min_resource_health_score: 0.35,
        }
    }
}

/// Deterministic kill decision.
#[derive(Debug, Clone)]
pub struct KillDecision {
    pub subtask_id: String,
    pub branch: String,
    pub reason: String,
}

/// One collapse sampling tick output.
#[derive(Debug, Clone, Default)]
pub struct CollapseTick {
    pub evaluations: Vec<BranchEvaluation>,
    pub kills: Vec<KillDecision>,
    pub promoted_subtask_id: Option<String>,
}

/// First-class regulator that samples branch health and applies deterministic collapse policy.
#[derive(Debug)]
pub struct CollapseController {
    policy: CollapsePolicy,
    first_seen_at: HashMap<String, Instant>,
    consecutive_compile_failures: HashMap<String, u32>,
    consecutive_resource_failures: HashMap<String, u32>,
    invariant_breaks: HashMap<String, u32>,
    promoted_subtask_id: Option<String>,
}

impl CollapseController {
    pub fn new(policy: CollapsePolicy) -> Self {
        Self {
            policy,
            first_seen_at: HashMap::new(),
            consecutive_compile_failures: HashMap::new(),
            consecutive_resource_failures: HashMap::new(),
            invariant_breaks: HashMap::new(),
            promoted_subtask_id: None,
        }
    }

    pub fn sample(&mut self, branches: &[BranchRuntimeState]) -> Result<CollapseTick> {
        if branches.is_empty() {
            return Ok(CollapseTick::default());
        }

        let mut probe_handles = Vec::with_capacity(branches.len());
        for branch in branches {
            let subtask_id = branch.subtask_id.clone();
            let branch_name = branch.branch.clone();
            let worktree_path = branch.worktree_path.clone();
            let handle = std::thread::spawn(move || -> Result<BranchObservation> {
                Ok(BranchObservation {
                    subtask_id,
                    branch: branch_name,
                    compile_ok: run_cargo_check(&worktree_path)?,
                    changed_files: collect_changed_files(&worktree_path)?,
                    changed_lines: collect_changed_lines(&worktree_path)?,
                    resource_health_score: 1.0,
                    infra_unhealthy_signals: 0,
                })
            });
            probe_handles.push(handle);
        }

        let mut observations = Vec::with_capacity(branches.len());
        for handle in probe_handles {
            let observation = handle
                .join()
                .map_err(|_| anyhow!("Branch sampling thread panicked"))??;
            observations.push(observation);
        }
        Ok(self.sample_observations(&observations))
    }

    pub fn sample_observations(&mut self, observations: &[BranchObservation]) -> CollapseTick {
        if observations.is_empty() {
            return CollapseTick::default();
        }

        let active: HashSet<&str> = observations.iter().map(|b| b.subtask_id.as_str()).collect();
        self.first_seen_at
            .retain(|id, _| active.contains(id.as_str()));
        self.consecutive_compile_failures
            .retain(|id, _| active.contains(id.as_str()));
        self.consecutive_resource_failures
            .retain(|id, _| active.contains(id.as_str()));
        self.invariant_breaks
            .retain(|id, _| active.contains(id.as_str()));

        let mut changed_files: HashMap<String, HashSet<String>> = HashMap::new();
        let mut changed_lines: HashMap<String, u32> = HashMap::new();
        let mut compile_ok: HashMap<String, bool> = HashMap::new();

        for observation in observations {
            self.first_seen_at
                .entry(observation.subtask_id.clone())
                .or_insert_with(Instant::now);
            changed_files.insert(
                observation.subtask_id.clone(),
                observation.changed_files.clone(),
            );
            changed_lines.insert(observation.subtask_id.clone(), observation.changed_lines);
            compile_ok.insert(observation.subtask_id.clone(), observation.compile_ok);

            let fail_counter = self
                .consecutive_compile_failures
                .entry(observation.subtask_id.clone())
                .or_insert(0);
            if observation.compile_ok {
                *fail_counter = 0;
            } else {
                *fail_counter += 1;
                *self
                    .invariant_breaks
                    .entry(observation.subtask_id.clone())
                    .or_insert(0) += 1;
            }

            let infra_fail_counter = self
                .consecutive_resource_failures
                .entry(observation.subtask_id.clone())
                .or_insert(0);
            if observation.resource_health_score < self.policy.min_resource_health_score {
                *infra_fail_counter += 1;
            } else {
                *infra_fail_counter = 0;
            }
            if observation.infra_unhealthy_signals > 0 {
                *self
                    .invariant_breaks
                    .entry(observation.subtask_id.clone())
                    .or_insert(0) += observation.infra_unhealthy_signals;
            }
        }

        let mut evaluations = Vec::with_capacity(observations.len());
        for observation in observations {
            let my_files = changed_files
                .get(&observation.subtask_id)
                .cloned()
                .unwrap_or_default();

            let overlap_files = changed_files
                .iter()
                .filter(|(other_id, _)| *other_id != &observation.subtask_id)
                .map(|(_, files)| my_files.intersection(files).count())
                .sum::<usize>();

            let conflict_risk = if my_files.is_empty() {
                0.0
            } else {
                (overlap_files as f32 / my_files.len() as f32).clamp(0.0, 1.0)
            };

            let compile_health = if *compile_ok.get(&observation.subtask_id).unwrap_or(&false) {
                1.0
            } else {
                0.0
            };

            // Phase 0 keeps test health lightweight to avoid expensive frequent test loops.
            let test_health = if compile_health > 0.0 { 0.6 } else { 0.0 };
            let contract_alignment = if compile_health > 0.0 {
                (1.0 - conflict_risk).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let elapsed_secs = self
                .first_seen_at
                .get(&observation.subtask_id)
                .map(|t| t.elapsed().as_secs_f32().max(1.0))
                .unwrap_or(1.0);
            let lines = changed_lines
                .get(&observation.subtask_id)
                .copied()
                .unwrap_or(0) as f32;
            let velocity = (lines / elapsed_secs / 20.0).clamp(0.0, 1.0);

            let invariant_breaks = *self
                .invariant_breaks
                .get(&observation.subtask_id)
                .unwrap_or(&0);
            let score = CoherenceScore {
                compile_health,
                test_health,
                contract_alignment,
                diff_conflict_risk: conflict_risk,
                velocity,
                resource_health_score: observation.resource_health_score.clamp(0.0, 1.0),
                invariant_breaks,
            };

            evaluations.push(BranchEvaluation {
                subtask_id: observation.subtask_id.clone(),
                branch: observation.branch.clone(),
                aggregate_score: score.collapse_score(),
                score,
            });
        }

        self.derive_decisions(evaluations)
    }

    fn derive_decisions(&mut self, evaluations: Vec<BranchEvaluation>) -> CollapseTick {
        if evaluations.is_empty() {
            self.promoted_subtask_id = None;
            return CollapseTick::default();
        }

        let mut kills = Vec::new();
        let mut killed_ids = HashSet::new();

        for eval in &evaluations {
            let consecutive_failures = *self
                .consecutive_compile_failures
                .get(&eval.subtask_id)
                .unwrap_or(&0);
            let consecutive_resource_failures = *self
                .consecutive_resource_failures
                .get(&eval.subtask_id)
                .unwrap_or(&0);
            if eval.score.invariant_breaks >= self.policy.max_invariant_breaks
                || consecutive_failures >= self.policy.max_consecutive_compile_failures
                || consecutive_resource_failures >= self.policy.max_consecutive_resource_failures
            {
                killed_ids.insert(eval.subtask_id.clone());
                kills.push(KillDecision {
                    subtask_id: eval.subtask_id.clone(),
                    branch: eval.branch.clone(),
                    reason: format!(
                        "policy_threshold exceeded (invariant_breaks={}, consecutive_compile_failures={}, consecutive_resource_failures={}, resource_health_score={:.3})",
                        eval.score.invariant_breaks, consecutive_failures, consecutive_resource_failures, eval.score.resource_health_score
                    ),
                });
            }
        }

        // Never collapse every branch in a single sampling tick.
        if killed_ids.len() == evaluations.len() {
            if let Some(best) = evaluations
                .iter()
                .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
            {
                killed_ids.remove(&best.subtask_id);
                kills.retain(|k| k.subtask_id != best.subtask_id);
            }
        }

        let alive: Vec<&BranchEvaluation> = evaluations
            .iter()
            .filter(|e| !killed_ids.contains(&e.subtask_id))
            .collect();

        // If enough candidates remain, prune one low-coherence branch.
        if alive.len() > 1 {
            let best = alive
                .iter()
                .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
                .copied();
            let worst = alive
                .iter()
                .min_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
                .copied();

            if let (Some(best), Some(worst)) = (best, worst) {
                let gap = best.aggregate_score - worst.aggregate_score;
                if worst.aggregate_score <= self.policy.min_collapse_score
                    && gap >= self.policy.min_score_gap_for_prune
                {
                    let newly_killed = killed_ids.insert(worst.subtask_id.clone());
                    if newly_killed {
                        kills.push(KillDecision {
                            subtask_id: worst.subtask_id.clone(),
                            branch: worst.branch.clone(),
                            reason: format!(
                                "low coherence pruned (score={:.3}, best={:.3}, gap={:.3})",
                                worst.aggregate_score, best.aggregate_score, gap
                            ),
                        });
                    }
                }
            }
        }

        let promoted_subtask_id = evaluations
            .iter()
            .filter(|e| !killed_ids.contains(&e.subtask_id))
            .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
            .map(|e| e.subtask_id.clone());

        self.promoted_subtask_id = promoted_subtask_id.clone();

        CollapseTick {
            evaluations,
            kills,
            promoted_subtask_id,
        }
    }
}

fn run_cargo_check(worktree_path: &PathBuf) -> Result<bool> {
    let mut child = Command::new("cargo")
        .args(["check", "--quiet"])
        .current_dir(worktree_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| {
            format!(
                "Failed to execute cargo check in {}",
                worktree_path.display()
            )
        })?;

    let deadline = Instant::now() + Duration::from_secs(COLLAPSE_CHECK_TIMEOUT_SECS);
    loop {
        if let Some(status) = child.try_wait().with_context(|| {
            format!(
                "Failed waiting on cargo check in {}",
                worktree_path.display()
            )
        })? {
            return Ok(status.success());
        }

        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            tracing::warn!(
                worktree_path = %worktree_path.display(),
                timeout_secs = COLLAPSE_CHECK_TIMEOUT_SECS,
                "Collapse sampling cargo check timed out"
            );
            return Ok(false);
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

const COLLAPSE_CHECK_TIMEOUT_SECS: u64 = 45;

fn collect_changed_files(worktree_path: &PathBuf) -> Result<HashSet<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only"])
        .current_dir(worktree_path)
        .output()
        .with_context(|| {
            format!(
                "Failed to collect changed files in {}",
                worktree_path.display()
            )
        })?;
    if !output.status.success() {
        return Ok(HashSet::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect())
}

fn collect_changed_lines(worktree_path: &PathBuf) -> Result<u32> {
    let output = Command::new("git")
        .args(["diff", "--numstat"])
        .current_dir(worktree_path)
        .output()
        .with_context(|| {
            format!(
                "Failed to collect changed lines in {}",
                worktree_path.display()
            )
        })?;
    if !output.status.success() {
        return Ok(0);
    }
    let mut total = 0u32;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        total += parts[0].parse::<u32>().unwrap_or(0);
        total += parts[1].parse::<u32>().unwrap_or(0);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapse_score_penalizes_invariants() {
        let healthy = CoherenceScore {
            compile_health: 1.0,
            test_health: 0.8,
            contract_alignment: 0.9,
            diff_conflict_risk: 0.1,
            velocity: 0.5,
            resource_health_score: 1.0,
            invariant_breaks: 0,
        };
        let broken = CoherenceScore {
            invariant_breaks: 3,
            ..healthy
        };

        assert!(healthy.collapse_score() > broken.collapse_score());
    }

    #[test]
    fn derive_decisions_kills_repeated_failures_and_promotes_best() {
        let mut controller = CollapseController::new(CollapsePolicy::default());
        controller
            .consecutive_compile_failures
            .insert("bad".to_string(), 3);
        controller.invariant_breaks.insert("bad".to_string(), 2);

        let evals = vec![
            BranchEvaluation {
                subtask_id: "bad".to_string(),
                branch: "codetether/subagent-bad".to_string(),
                score: CoherenceScore {
                    compile_health: 0.0,
                    test_health: 0.0,
                    contract_alignment: 0.0,
                    diff_conflict_risk: 0.9,
                    velocity: 0.1,
                    resource_health_score: 0.2,
                    invariant_breaks: 2,
                },
                aggregate_score: 0.0,
            },
            BranchEvaluation {
                subtask_id: "good".to_string(),
                branch: "codetether/subagent-good".to_string(),
                score: CoherenceScore {
                    compile_health: 1.0,
                    test_health: 0.6,
                    contract_alignment: 0.9,
                    diff_conflict_risk: 0.1,
                    velocity: 0.4,
                    resource_health_score: 1.0,
                    invariant_breaks: 0,
                },
                aggregate_score: 0.82,
            },
        ];

        let tick = controller.derive_decisions(evals);
        assert_eq!(tick.kills.len(), 1);
        assert_eq!(tick.kills[0].subtask_id, "bad");
        assert_eq!(tick.promoted_subtask_id.as_deref(), Some("good"));
    }

    #[test]
    fn sample_observations_penalizes_resource_unhealth() {
        let mut controller = CollapseController::new(CollapsePolicy::default());
        let observations = vec![
            BranchObservation {
                subtask_id: "infra-bad".to_string(),
                branch: "codetether/subagent-infra-bad".to_string(),
                compile_ok: true,
                changed_files: HashSet::new(),
                changed_lines: 1,
                resource_health_score: 0.0,
                infra_unhealthy_signals: 2,
            },
            BranchObservation {
                subtask_id: "infra-good".to_string(),
                branch: "codetether/subagent-infra-good".to_string(),
                compile_ok: true,
                changed_files: HashSet::new(),
                changed_lines: 1,
                resource_health_score: 1.0,
                infra_unhealthy_signals: 0,
            },
        ];

        let tick = controller.sample_observations(&observations);
        let bad = tick
            .evaluations
            .iter()
            .find(|e| e.subtask_id == "infra-bad")
            .expect("infra-bad evaluation");
        let good = tick
            .evaluations
            .iter()
            .find(|e| e.subtask_id == "infra-good")
            .expect("infra-good evaluation");
        assert!(bad.aggregate_score < good.aggregate_score);
    }
}
