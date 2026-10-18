//! Original in-memory backend behavior tests.
use super::*;
use crate::{QualityChecks, RalphConfig, UserStory};
mod fixtures;
use fixtures::test_run_state;
mod test_complete_run;
mod test_create_and_get_run;
mod test_get_nonexistent_run;
mod test_list_runs;
mod test_record_story_result;
mod test_update_status;
