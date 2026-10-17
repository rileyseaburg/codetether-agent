//! Oracle-friendly query metadata, rendering, and ordered catalog access.

mod catalog;
mod data;
mod generated;
mod iterate;
mod kind;
mod query;
mod render;
use catalog::TEMPLATES;
pub use data::{Param, QueryTemplate};
pub use generated::GeneratedQuery;
pub use iterate::TemplateIter;
pub use kind::TemplateKind;
#[cfg(test)]
mod tests;
