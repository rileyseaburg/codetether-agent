//! Ordered LaTeX-to-Unicode substitutions for terminal math text.
//!
//! [prettify_math] replaces known commands without typesetting or parsing a
//! full expression. Unknown commands and the original substitution order
//! are preserved. Tables stay private to keep one conversion contract.
mod replace;
mod table_1;
mod table_2;
mod table_3;
pub use replace::prettify_math;
#[cfg(test)]
mod tests;
