//! Shared JSON/TetherScript value conversion for tools and providers.
//!
//! [json_to_tetherscript] preserves oversized integers as strings;
//! [tetherscript_to_json] serializes runtime results and descriptive resources.
mod from_json;
mod to_json;

pub use from_json::json_to_tetherscript;
pub use to_json::tetherscript_to_json;

#[cfg(test)]
mod tests;
