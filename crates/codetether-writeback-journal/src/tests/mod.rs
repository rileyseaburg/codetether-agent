//! Original journal state-machine and serde regressions.
mod journal_entry_round_trips_through_serde;
mod rejected_writeback_does_not_commit;
mod stage_then_commit_appends_two_entries;
mod txn_ids_are_monotonic;
