//! Selectable derivation policies for [`derive_with_policy`].
//!
//! [`derive_context`] in Phase A is a single fixed pipeline: clone +
//! experimental + `enforce_on_messages` + pairing repair. Phase B
//! parameterises the pipeline with a [`DerivePolicy`] so different
//! research directions can share the same `&Session → DerivedContext`
//! signature.
//!
//! ## Variants
//!
//! * [`DerivePolicy::Legacy`] — the Phase A behaviour. Current default
//!   until the Pareto benchmark ([plan step 23]) demonstrates one of
//!   the alternatives is dominant.
//! * [`DerivePolicy::Reset`] — **Lu et al.** reset-to-(prompt, summary)
//!   semantic from arXiv:2510.06727. When the token estimate exceeds
//!   the threshold, compresses the prefix to a single summary message
//!   and keeps only the most recent user turn. See
//!   [`derive_with_policy`](super::context::derive_with_policy) for
//!   the implementation.
//! * [`DerivePolicy::Incremental`] *(reserved, Phase B)* — Liu et al.
//!   scoring + hierarchical summary lookup.
//! * [`DerivePolicy::OracleReplay`] *(reserved, Phase B)* — ClawVM
//!   replay oracle with `h`-turn future-demand lookahead.
//!
//! [`derive_context`]: super::context::derive_context
//! [`derive_with_policy`]: super::context::derive_with_policy
//! [plan step 23]: crate::session::context
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::derive_policy::DerivePolicy;
//!
//! let legacy = DerivePolicy::Legacy;
//! let reset = DerivePolicy::Reset { threshold_tokens: 16_000 };
//!
//! assert!(matches!(legacy, DerivePolicy::Legacy));
//! assert!(matches!(reset, DerivePolicy::Reset { .. }));
//! ```

use serde::{Deserialize, Serialize};

/// Per-session derivation strategy selector.
///
/// Defaults to [`DerivePolicy::Legacy`] so existing code paths that do
/// not opt into a new policy get the Phase A behaviour unchanged.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "policy", rename_all = "snake_case")]
pub enum DerivePolicy {
    /// Phase A clone + experimental + `enforce_on_messages` + pairing.
    /// The historical pipeline.
    Legacy,
    /// Lu et al. (arXiv:2510.06727) reset-to-(prompt, summary).
    ///
    /// When the request's estimated token cost exceeds
    /// `threshold_tokens`, replace everything older than the last user
    /// turn with a single RLM-generated summary and discard the rest of
    /// the tail. The derived context for the next provider call
    /// contains at most `[summary, last_user_turn]`.
    Reset {
        /// Token budget that triggers the reset. Typically ~95 % of
        /// the model's working context length.
        threshold_tokens: usize,
    },
}

impl DerivePolicy {
    /// Human-readable short name for logs, journal entries, and
    /// telemetry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::derive_policy::DerivePolicy;
    ///
    /// assert_eq!(DerivePolicy::Legacy.kind(), "legacy");
    /// assert_eq!(
    ///     DerivePolicy::Reset { threshold_tokens: 0 }.kind(),
    ///     "reset",
    /// );
    /// ```
    pub fn kind(&self) -> &'static str {
        match self {
            DerivePolicy::Legacy => "legacy",
            DerivePolicy::Reset { .. } => "reset",
        }
    }
}

impl Default for DerivePolicy {
    fn default() -> Self {
        DerivePolicy::Legacy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_legacy() {
        assert!(matches!(DerivePolicy::default(), DerivePolicy::Legacy));
    }

    #[test]
    fn reset_carries_threshold() {
        let p = DerivePolicy::Reset {
            threshold_tokens: 8192,
        };
        if let DerivePolicy::Reset { threshold_tokens } = p {
            assert_eq!(threshold_tokens, 8192);
        } else {
            panic!("expected Reset");
        }
    }

    #[test]
    fn kind_is_snake_case_and_distinct() {
        assert_eq!(DerivePolicy::Legacy.kind(), "legacy");
        assert_eq!(
            DerivePolicy::Reset {
                threshold_tokens: 0
            }
            .kind(),
            "reset"
        );
    }

    #[test]
    fn policy_round_trips_through_serde() {
        let p = DerivePolicy::Reset {
            threshold_tokens: 12_000,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"policy\":\"reset\""));
        let back: DerivePolicy = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            back,
            DerivePolicy::Reset {
                threshold_tokens: 12_000
            }
        ));
    }
}
