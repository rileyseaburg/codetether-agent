//! Belief lifecycle regression cases.
use super::*;
use chrono::{Duration, Utc};
mod confidence_clamping;
mod contradiction_tracking;
mod decay_reduces_confidence;
mod duplicate_detection_by_belief_key;
mod revalidation_failure_decreases_confidence;
mod revalidation_success_increases_confidence;
