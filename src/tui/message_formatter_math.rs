//! LaTeX-to-Unicode substitution for inline and display math.
//!
//! Split from [`super::message_formatter`] so that module stays within the
//! file budget. Maps common LaTeX commands to their Unicode glyphs, applied
//! longest-first so `\delta` is never shortened to `\d`. The replacement
//! table itself is sharded across `message_formatter_math_{1,2,3}` to keep
//! every file under the line budget.

#[path = "message_formatter_math_1.rs"]
mod table_1;
#[path = "message_formatter_math_2.rs"]
mod table_2;
#[path = "message_formatter_math_3.rs"]
mod table_3;

/// Replace known LaTeX math commands in `input` with Unicode equivalents.
///
/// Pairs are applied longest-first (table ordering) to avoid e.g. `\delta`
/// matching `\d`. Unknown macros are left untouched.
pub(super) fn prettify_math(input: &str) -> String {
    let mut out = input.to_string();
    let tables = [table_1::PAIRS, table_2::PAIRS, table_3::PAIRS];
    for (from, to) in tables.into_iter().flatten() {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    out
}
