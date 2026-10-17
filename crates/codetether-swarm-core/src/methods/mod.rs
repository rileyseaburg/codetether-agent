//! Pure metrics calculations and rate-limit header interpretation.
//!
//! These implementations operate on shared contracts and standard time/types.
use crate::*;
use std::{collections::HashMap, time::Duration};
mod cache_stats_hit_rate;
mod cache_stats_record_hit;
mod cache_stats_record_miss;
mod cache_stats_total_lookups;
mod rate_limit_info_from_headers;
mod rate_limit_info_is_approaching_limit;
mod rate_limit_info_is_limit_exceeded;
mod rate_limit_info_recommended_delay;
mod swarm_stats_cache_hit_rate;
mod swarm_stats_calculate_critical_path;
mod swarm_stats_calculate_speedup;
mod swarm_stats_merge_cache_stats;
