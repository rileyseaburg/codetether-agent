//! Direct-backend mutation authorization tests.

#[path = "direct_mutation_preview_tests.rs"]
mod preview;
#[path = "direct_mutation_write_tests.rs"]
mod write;
#[path = "direct_mutation_approval_test_support.rs"]
mod support;
#[path = "direct_mutation_advanced_edit_tests.rs"]
mod advanced_edit;
#[path = "direct_mutation_confirm_tests.rs"]
mod confirm;
#[path = "direct_mutation_confirm_scope_tests.rs"]
mod confirm_scope;
#[path = "direct_mutation_confirm_retry_tests.rs"]
mod confirm_retry;
#[path = "direct_mutation_confirm_multi_tests.rs"]
mod confirm_multi;
#[path = "direct_mutation_todo_tests.rs"]
mod todo;
#[path = "direct_mutation_todo_alias_tests.rs"]
mod todo_alias;
#[path = "direct_mutation_session_grant_tests.rs"]
mod session_grant;
#[path = "direct_mutation_todo_branch_tests.rs"]
mod todo_branches;