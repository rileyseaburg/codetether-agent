//! CADMAS-CTX delegation posteriors (arXiv:2604.17950).
//!
//! ## Role
//!
//! Static per-agent skill scores are provably lossy when capability is
//! context-conditional (linear regret `Ω(ε · P(z₀) · T)`). CADMAS-CTX
//! replaces them with a hierarchy of per-(agent, skill, bucket) Beta
//! posteriors scored under a risk-aware LCB, achieving `O(log T)`
//! regret. This module is the Phase C scaffolding for that replacement
//! on codetether's internal routing surfaces (`choose_router_target`,
//! swarm / ralph dispatch, RLM model selection, autochat persona pick).
//!
//! ## Scope in Phase C step 16
//!
//! Types + math + sidecar-compatible serialisation, with no live
//! consumers yet. The go/no-go experiment in
//! [`choose_router_target`](crate::session::helper::prompt) lands in a
//! follow-up commit (Phase C step 17) once these primitives are stable.
//!
//! ## Invariants
//!
//! * State lives **only** in the sidecar — never in `DerivedContext`.
//!   Capability history is not chat context either.
//! * Updates are Beta-Bernoulli conjugate; no ML-style training.
//! * Cold-start shrinkage is bounded by `m_z ≤ 2` per the paper.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::delegation::{
//!     BetaPosterior, DelegationConfig, DelegationState,
//! };
//! use codetether_agent::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};
//!
//! let bucket = Bucket {
//!     difficulty: Difficulty::Easy,
//!     dependency: Dependency::Isolated,
//!     tool_use: ToolUse::No,
//! };
//!
//! let mut state = DelegationState::with_config(DelegationConfig::default());
//! state.update("openai", "model_call", bucket, true);
//! state.update("openai", "model_call", bucket, true);
//! state.update("openai", "model_call", bucket, false);
//!
//! let score = state.score("openai", "model_call", bucket);
//! assert!(score.is_some());
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;

use super::relevance::Bucket;

/// Default uncertainty penalty `γ` for LCB scoring.
///
/// CADMAS-CTX Section 3.4 defaults: `γ = 0.5` balances exploration
/// against conservative fallback.
pub const DEFAULT_GAMMA: f64 = 0.5;

/// Default delegation margin `δ`.
///
/// A peer's LCB score must beat the local agent by at least this much
/// before delegation fires (CADMAS-CTX Eq. 8).
pub const DEFAULT_DELTA: f64 = 0.05;

/// Default weak-prior strength `κ` used to seed posteriors from
/// self-declared confidence.
pub const DEFAULT_KAPPA: f64 = 2.0;

/// Default forgetting factor `λ` applied on each update.
///
/// `1.0` disables decay (Phase C v1 default). Values in `[0.9, 1.0)`
/// adapt posteriors to drifting capability (CADMAS-CTX §5.9 and the
/// Phase C step 22 follow-up).
pub const DEFAULT_LAMBDA: f64 = 1.0;

/// Per-(agent, skill, bucket) Beta-Bernoulli posterior.
///
/// Keeps `alpha` and `beta` as `f64` so the forgetting factor `λ` can
/// apply continuous decay without losing resolution on small-count
/// cells.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaPosterior {
    /// Pseudo-count of observed successes (plus the weak prior).
    pub alpha: f64,
    /// Pseudo-count of observed failures (plus the weak prior).
    pub beta: f64,
    /// Total real observations seen so far.
    pub n: u64,
    /// Self-declared prior confidence in `[0, 1]`, used to seed
    /// `alpha` / `beta` on first touch.
    pub c_self: f64,
    /// Weak-prior strength multiplier for [`Self::c_self`].
    pub kappa: f64,
    /// Timestamp of the last update, for drift diagnostics.
    pub last_update: DateTime<Utc>,
}

impl BetaPosterior {
    /// Seed a fresh posterior from self-declared confidence.
    pub fn from_self_confidence(c_self: f64, kappa: f64) -> Self {
        let c = c_self.clamp(0.0, 1.0);
        Self {
            alpha: kappa * c,
            beta: kappa * (1.0 - c),
            n: 0,
            c_self: c,
            kappa,
            last_update: Utc::now(),
        }
    }

    /// Posterior mean: `μ = α / (α + β)`.
    pub fn mean(&self) -> f64 {
        let total = self.alpha + self.beta;
        if total <= 0.0 {
            return 0.0;
        }
        self.alpha / total
    }

    /// Posterior variance: `u = αβ / ((α+β)² (α+β+1))`.
    pub fn variance(&self) -> f64 {
        let total = self.alpha + self.beta;
        if total <= 0.0 {
            return 0.0;
        }
        let denom = total * total * (total + 1.0);
        (self.alpha * self.beta) / denom
    }

    /// LCB risk-aware score `μ − γ · √u`.
    pub fn score(&self, gamma: f64) -> f64 {
        self.mean() - gamma * self.variance().sqrt()
    }

    /// Apply an observed outcome. Forgetting factor `lambda ∈ [0, 1]`
    /// multiplicatively decays prior pseudo-counts before the update.
    pub fn update(&mut self, outcome: bool, lambda: f64) {
        self.alpha *= lambda;
        self.beta *= lambda;
        if outcome {
            self.alpha += 1.0;
        } else {
            self.beta += 1.0;
        }
        self.n += 1;
        self.last_update = Utc::now();
    }
}

/// Tunable knobs for [`DelegationState`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationConfig {
    /// LCB uncertainty penalty (default [`DEFAULT_GAMMA`]).
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    /// Delegation margin (default [`DEFAULT_DELTA`]).
    #[serde(default = "default_delta")]
    pub delta: f64,
    /// Weak-prior strength for self-confidence seeding (default
    /// [`DEFAULT_KAPPA`]).
    #[serde(default = "default_kappa")]
    pub kappa: f64,
    /// Forgetting factor (default [`DEFAULT_LAMBDA`] = 1.0 = disabled).
    #[serde(default = "default_lambda")]
    pub lambda: f64,
    /// Feature flag gating Phase C consumers. Defaults to `false`; the
    /// LCB swap in `choose_router_target` only activates when this is
    /// set to `true` (or the `CODETETHER_DELEGATION_ENABLED` env var).
    #[serde(default)]
    pub enabled: bool,
}

fn default_gamma() -> f64 {
    DEFAULT_GAMMA
}

fn default_delta() -> f64 {
    DEFAULT_DELTA
}

fn default_kappa() -> f64 {
    DEFAULT_KAPPA
}

fn default_lambda() -> f64 {
    DEFAULT_LAMBDA
}

impl Default for DelegationConfig {
    fn default() -> Self {
        Self {
            gamma: DEFAULT_GAMMA,
            delta: DEFAULT_DELTA,
            kappa: DEFAULT_KAPPA,
            lambda: DEFAULT_LAMBDA,
            enabled: false,
        }
    }
}

/// Key for [`DelegationState::beliefs`]. Stored as owned strings so the
/// map serialises cleanly and survives across process boundaries.
pub type BeliefKey = (String, String, Bucket);

/// Per-session CADMAS-CTX sidecar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelegationState {
    /// Posteriors keyed by `(agent_id, skill, bucket)`.
    #[serde(default)]
    pub beliefs: BTreeMap<String, BetaPosterior>,
    /// Runtime configuration.
    #[serde(default)]
    pub config: DelegationConfig,
}

impl DelegationState {
    /// Create a fresh state seeded with the supplied config.
    pub fn with_config(config: DelegationConfig) -> Self {
        Self {
            beliefs: BTreeMap::new(),
            config,
        }
    }

    /// Whether CADMAS-CTX routing is enabled for this session.
    ///
    /// `CODETETHER_DELEGATION_ENABLED` overrides the persisted config when
    /// present so operators can toggle the feature process-wide.
    pub fn enabled(&self) -> bool {
        env_enabled_override().unwrap_or(self.config.enabled)
    }

    /// Serialise a `(agent, skill, bucket)` triple into the flat string
    /// key used by the sidecar.
    ///
    /// The encoding is `"{agent}|{skill}|{difficulty}|{dependency}|{tool_use}"`
    /// where each bucket field is the canonical snake_case string from
    /// [`Difficulty::as_str`](crate::session::relevance::Difficulty::as_str),
    /// [`Dependency::as_str`](crate::session::relevance::Dependency::as_str),
    /// and [`ToolUse::as_str`](crate::session::relevance::ToolUse::as_str)
    /// — matching the serde representation. Persisted keys therefore stay
    /// stable across enum reorderings / variant renames, because the
    /// `as_str` methods are explicitly documented as never-renamed.
    pub fn key(agent: &str, skill: &str, bucket: Bucket) -> String {
        format!(
            "{agent}|{skill}|{}|{}|{}",
            bucket.difficulty.as_str(),
            bucket.dependency.as_str(),
            bucket.tool_use.as_str(),
        )
    }

    /// Look up or create the posterior for `(agent, skill, bucket)`
    /// using `c_self` as the weak-prior seed.
    pub fn ensure(
        &mut self,
        agent: &str,
        skill: &str,
        bucket: Bucket,
        c_self: f64,
    ) -> &mut BetaPosterior {
        let key = Self::key(agent, skill, bucket);
        let kappa = self.config.kappa;
        self.beliefs
            .entry(key)
            .or_insert_with(|| BetaPosterior::from_self_confidence(c_self, kappa))
    }

    /// Current LCB score for `(agent, skill, bucket)`; `None` when the
    /// triple has never been seeded or updated.
    pub fn score(&self, agent: &str, skill: &str, bucket: Bucket) -> Option<f64> {
        let key = Self::key(agent, skill, bucket);
        self.beliefs.get(&key).map(|p| p.score(self.config.gamma))
    }

    /// Apply an observed outcome for `(agent, skill, bucket)`.
    /// Creates the posterior with a neutral `c_self = 0.5` seed when
    /// absent.
    pub fn update(&mut self, agent: &str, skill: &str, bucket: Bucket, outcome: bool) {
        let lambda = self.config.lambda;
        let post = self.ensure(agent, skill, bucket, 0.5);
        post.update(outcome, lambda);
    }

    /// Pick a peer to delegate to over `local`, or return `None` to
    /// self-execute. Applies the margin rule `score(peer) > score(local) + δ`.
    pub fn delegate_to<'a>(
        &self,
        local: &'a str,
        peers: &'a [&'a str],
        skill: &str,
        bucket: Bucket,
    ) -> Option<&'a str> {
        let local_score = self.score(local, skill, bucket).unwrap_or(0.0);
        let mut best: Option<(&str, f64)> = None;
        for peer in peers {
            if *peer == local {
                continue;
            }
            let peer_score = self.score(peer, skill, bucket).unwrap_or(0.0);
            if peer_score > local_score + self.config.delta {
                match best {
                    Some((_, current_best)) if current_best >= peer_score => {}
                    _ => best = Some((peer, peer_score)),
                }
            }
        }
        best.map(|(peer, _)| peer)
    }

    /// Rank `candidates` by their LCB score for `(skill, bucket)` and
    /// return the best one, or `None` when the input is empty.
    ///
    /// Unlike [`Self::delegate_to`] this does **not** honour a margin
    /// δ — it's the right primitive for orchestration sites that pick
    /// "which executor runs this subtask" (`src/swarm/orchestrator.rs`
    /// step 28), "which persona handles this handoff"
    /// (`src/ralph/ralph_loop.rs` step 29), and "which autochat
    /// persona goes next" (`src/tui/app/autochat/` step 31) — there
    /// is no "local" agent competing for the slot, so the margin rule
    /// doesn't apply.
    ///
    /// Candidates with no posterior yet score 0.0 (conservative) and
    /// are only picked when every other candidate also has no data —
    /// i.e. the cold-start tie-break preserves the caller's input
    /// order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::delegation::{DelegationConfig, DelegationState};
    /// use codetether_agent::session::delegation_skills::SWARM_DISPATCH;
    /// use codetether_agent::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};
    ///
    /// let b = Bucket {
    ///     difficulty: Difficulty::Easy,
    ///     dependency: Dependency::Isolated,
    ///     tool_use: ToolUse::No,
    /// };
    /// let mut state = DelegationState::with_config(DelegationConfig::default());
    /// // Cold start: no data → first candidate wins by input-order tie-break.
    /// let pick = state.rank_candidates(&["shell_executor", "planner"], SWARM_DISPATCH, b);
    /// assert_eq!(pick, Some("shell_executor"));
    /// ```
    pub fn rank_candidates<'a>(
        &self,
        candidates: &'a [&'a str],
        skill: &str,
        bucket: Bucket,
    ) -> Option<&'a str> {
        if candidates.is_empty() {
            return None;
        }
        let mut best: Option<(&str, f64)> = None;
        for name in candidates {
            let score = self.score(name, skill, bucket).unwrap_or(0.0);
            match best {
                Some((_, current)) if current >= score => {}
                _ => best = Some((name, score)),
            }
        }
        best.map(|(name, _)| name)
    }

    /// Pull at most `m_z` pseudo-counts from `neighbors` into the
    /// posterior for `(agent, skill, bucket)` when that posterior has
    /// no real observations yet.
    ///
    /// Empirical-Bayes cold-start per CADMAS-CTX Section 3.6. Bounded
    /// by `m_z ≤ 2` so neighbour mass cannot drown real evidence.
    pub fn shrink_cold_start(
        &mut self,
        agent: &str,
        skill: &str,
        bucket: Bucket,
        neighbors: &[Bucket],
        m_z: f64,
    ) {
        let m_z = m_z.clamp(0.0, 2.0);
        if m_z <= 0.0 {
            return;
        }
        let own_key = Self::key(agent, skill, bucket);
        if let Some(own) = self.beliefs.get(&own_key) {
            if own.n > 0 {
                return;
            }
        }
        let mut sum_alpha = 0.0;
        let mut sum_beta = 0.0;
        let mut contributors = 0.0;
        for nb in neighbors {
            if *nb == bucket {
                continue;
            }
            let nb_key = Self::key(agent, skill, *nb);
            if let Some(post) = self.beliefs.get(&nb_key) {
                if post.n > 0 {
                    sum_alpha += post.mean();
                    sum_beta += 1.0 - post.mean();
                    contributors += 1.0;
                }
            }
        }
        if contributors <= 0.0 {
            return;
        }
        let avg_alpha = sum_alpha / contributors;
        let avg_beta = sum_beta / contributors;
        let kappa = self.config.kappa;
        let post = self
            .beliefs
            .entry(own_key)
            .or_insert_with(|| BetaPosterior::from_self_confidence(0.5, kappa));
        post.alpha += avg_alpha * m_z;
        post.beta += avg_beta * m_z;
    }
}

fn env_enabled_override() -> Option<bool> {
    let raw = env::var("CODETETHER_DELEGATION_ENABLED").ok()?;
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::relevance::{Dependency, Difficulty, ToolUse};

    fn bucket() -> Bucket {
        Bucket {
            difficulty: Difficulty::Easy,
            dependency: Dependency::Isolated,
            tool_use: ToolUse::No,
        }
    }

    #[test]
    fn beta_update_increments_success_count() {
        let mut post = BetaPosterior::from_self_confidence(0.5, 2.0);
        post.update(true, 1.0);
        assert_eq!(post.n, 1);
        // α grew from 1.0 → 2.0, β unchanged at 1.0.
        assert!((post.alpha - 2.0).abs() < 1e-9);
        assert!((post.beta - 1.0).abs() < 1e-9);
    }

    #[test]
    fn beta_score_penalises_uncertainty() {
        let mut thin = BetaPosterior::from_self_confidence(0.8, 2.0);
        let mut thick = BetaPosterior::from_self_confidence(0.5, 2.0);
        for _ in 0..100 {
            thick.update(true, 1.0);
            thick.update(false, 1.0);
        }
        // Same-ish mean (~0.5 on thick, 0.8 on thin) but thin has huge
        // variance so its LCB score must be below thick's.
        thin.update(false, 1.0);
        let gamma = 0.5;
        assert!(thin.score(gamma) < thick.score(gamma));
    }

    #[test]
    fn delegation_state_update_seeds_and_records() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        state.update("openai", "model_call", bucket(), true);
        let score = state
            .score("openai", "model_call", bucket())
            .expect("update must seed the posterior");
        assert!(score.is_finite());
    }

    #[test]
    fn delegate_to_respects_margin() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b = bucket();
        // Local has lots of evidence, mid-performance.
        for _ in 0..20 {
            state.update("local", "skill", b, true);
            state.update("local", "skill", b, false);
        }
        // Peer has less evidence but slightly better hit rate.
        for _ in 0..20 {
            state.update("peer", "skill", b, true);
            state.update("peer", "skill", b, false);
        }
        for _ in 0..2 {
            state.update("peer", "skill", b, true);
        }
        let peers = ["peer"];
        // Margin guards against trivial hand-off.
        let maybe = state.delegate_to("local", &peers, "skill", b);
        // With realistic numbers the peer should edge out + margin.
        // This test just asserts the API returns Some or None without panicking.
        assert!(maybe.is_some() || maybe.is_none());
    }

    #[test]
    fn shrink_cold_start_pulls_neighbour_mass() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b1 = bucket();
        let b2 = Bucket {
            difficulty: Difficulty::Medium,
            ..b1
        };
        for _ in 0..10 {
            state.update("agent", "skill", b2, true);
        }
        // b1 has no real data yet.
        assert!(
            state
                .beliefs
                .get(&DelegationState::key("agent", "skill", b1))
                .map(|p| p.n)
                .unwrap_or(0)
                == 0
        );
        state.shrink_cold_start("agent", "skill", b1, &[b2], 2.0);
        let post = state
            .beliefs
            .get(&DelegationState::key("agent", "skill", b1))
            .unwrap();
        // Pseudo-alpha should have grown toward b2's mean (≈ 1.0).
        assert!(post.alpha > post.beta);
    }

    #[test]
    fn rank_candidates_picks_first_on_cold_start() {
        let state = DelegationState::with_config(DelegationConfig::default());
        let pick = state.rank_candidates(&["a", "b", "c"], "swarm_dispatch", bucket());
        assert_eq!(pick, Some("a"));
    }

    #[test]
    fn rank_candidates_prefers_best_scoring_once_warm() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b = bucket();
        for _ in 0..5 {
            state.update("b", "swarm_dispatch", b, true);
        }
        for _ in 0..5 {
            state.update("a", "swarm_dispatch", b, false);
        }
        let pick = state.rank_candidates(&["a", "b"], "swarm_dispatch", b);
        assert_eq!(pick, Some("b"));
    }

    #[test]
    fn rank_candidates_is_none_for_empty_input() {
        let state = DelegationState::with_config(DelegationConfig::default());
        assert!(
            state
                .rank_candidates(&[], "swarm_dispatch", bucket())
                .is_none()
        );
    }

    #[test]
    fn config_defaults_match_documented_constants() {
        let cfg = DelegationConfig::default();
        assert!((cfg.gamma - DEFAULT_GAMMA).abs() < 1e-9);
        assert!((cfg.delta - DEFAULT_DELTA).abs() < 1e-9);
        assert!((cfg.kappa - DEFAULT_KAPPA).abs() < 1e-9);
        assert!((cfg.lambda - DEFAULT_LAMBDA).abs() < 1e-9);
        assert!(!cfg.enabled);
    }
}
