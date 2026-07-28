//! Layout-based hit testing.

pub mod point;
pub mod result;
pub mod search;

#[cfg(test)]
mod tests;

pub use point::point_in_box;
pub use result::HitResult;
pub use search::hit_test_layout;
