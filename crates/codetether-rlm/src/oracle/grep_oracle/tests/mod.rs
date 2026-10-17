//! Existing behavior regressions.

use super::*;
use crate::oracle::QueryType;
mod classify_pattern_match_query;
mod grep_finds_matches;
mod infer_async_pattern;
mod infer_pub_pattern;
mod infer_quoted_literal_pattern;
mod sample_rust_code;
mod verify_count_result;
mod verify_exact_match;

use sample_rust_code::sample_rust_code;
