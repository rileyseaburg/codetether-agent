//! Task relevance and dropped-range regressions.
use super::ranges::collect_dropped_ranges;
use super::scoring::overlap_count;
use super::*;
use codetether_provider_core::{ContentPart, Role};
mod fixtures;
use fixtures::*;
mod collect_dropped_ranges_empty_when_all_kept;
mod collect_dropped_ranges_groups_consecutive_drops;
mod overlap_count_short_circuits_on_empty;
mod score_messages_rewards_file_overlap;
mod task_signature_picks_up_files_from_last_user_turn;
