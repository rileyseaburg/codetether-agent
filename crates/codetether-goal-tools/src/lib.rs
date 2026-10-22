//! Model-facing goal operations with native storage bindings.
//!
//! [create] validates and creates an uncapped goal. [edit] strips injected
//! context before a revision-bound update. [Host] retains runtime reads and
//! controller updates; [response] renders the shared model-visible result.
pub mod context;
mod create;
mod create_run;
mod create_validate;
mod edit_run;
mod edit_schema;
mod host;
mod source;
pub use source::GoalSource;
pub mod response;
mod response_source;
pub use create::Args as CreateArgs;
pub use create_run::run as create;
pub use edit_run::run as edit;
pub use edit_schema::parameters as edit_parameters;
pub use host::Host;

mod create_tool;
mod edit_tool;
mod get_tool;
pub use create_tool::CreateGoalTool;
pub use edit_tool::EditGoalTool;
pub use get_tool::GetGoalTool;

mod update;
pub use update::{Args as UpdateArgs, Host as UpdateHost, UpdateGoalTool};
pub use update::{run as update, run_with as update_with};

#[cfg(test)]
mod source_tests;
