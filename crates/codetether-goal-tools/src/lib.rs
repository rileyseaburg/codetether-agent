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
pub mod response;
pub use create::Args as CreateArgs;
pub use create_run::run as create;
pub use edit_run::run as edit;
pub use edit_schema::parameters as edit_parameters;
pub use host::Host;
