//! Isolated benchmark project preparation.
//!
//! scaffold_workspace owns the temporary directory. Internal modules initialize
//! Rust dependencies and Git; dropping the returned handle removes the project.
mod dependencies;
mod initialize_git;
mod initialize_rust;
mod scaffold;
pub use scaffold::scaffold_workspace;
