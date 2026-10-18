//! Native regression coverage for prepared packing and active-turn clamping.
use crate as incremental_clamp;
use crate::select;
use codetether_context_policy::projection::incremental_insert::summary_message;
use codetether_context_policy::projection::incremental_types::{self, SummaryGap};
mod incremental_clamp_test_fixtures;
mod incremental_clamp_tests;
mod incremental_pack_tests;
