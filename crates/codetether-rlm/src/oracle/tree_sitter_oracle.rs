//! Tree-sitter oracle for structural AST verification.
//!
//! This oracle parses Rust source with tree-sitter and verifies structural
//! facts such as functions, structs, enums, impls, and error patterns.

mod answer_names;
mod classify;
mod compare;
mod definitions;
mod enums;
mod errors;
mod extract;
mod functions;
mod impls;
mod oracle;
mod query;
mod query_match;
mod structs;
mod types;
mod verification;
mod verify;
mod verify_targets;

pub use definitions::{
    EnumDefinition, ErrorPatternCounts, FunctionSignature, ImplDefinition, StructDefinition,
};
pub use oracle::TreeSitterOracle;
pub use types::{AstMatch, AstQueryResult};
pub use verification::TreeSitterVerification;

#[cfg(test)]
mod tests;
