//! Apply the original ordered tables of LaTeX-to-Unicode substitutions.
use crate::{table_1, table_2, table_3};

/// Replace known LaTeX math commands in `input` with Unicode equivalents.
///
/// Pairs are applied in table order. Unknown macros are left untouched.
/// # Arguments
/// * `input` - Text containing optional LaTeX commands.
/// # Returns
/// Owned text with known commands replaced in the existing table order.
/// # Examples
/// ```
/// assert_eq!(codetether_math_text::prettify_math("\\sum"), "Σ");
/// assert_eq!(codetether_math_text::prettify_math("\\unknownmacro"), "\\unknownmacro");
/// ```
pub fn prettify_math(input: &str) -> String {
    let mut out = input.to_string();
    let tables = [table_1::PAIRS, table_2::PAIRS, table_3::PAIRS];
    for (from, to) in tables.into_iter().flatten() {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    out
}
