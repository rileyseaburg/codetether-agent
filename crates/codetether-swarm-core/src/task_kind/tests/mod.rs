//! Existing intent classification tests.
use super::{TaskKind, classify};
mod design_review_is_read_only;
mod explicit_mutation_wins_over_review_language;
mod read_only_context_does_not_trigger_mutation_substrings;
mod schema_examples_have_non_mutating_intent;
