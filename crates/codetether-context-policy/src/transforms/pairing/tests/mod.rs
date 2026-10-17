//! Pairing regression tests.
use super::*;
mod fixtures;
use fixtures::*;
mod mixed_orphan_and_valid_pairs;
mod orphan_call_gets_synthetic_result;
mod orphan_result_is_dropped;
mod well_formed_history_is_noop;
