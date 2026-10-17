//! Apply the first unambiguous replacement in strategy order.
use super::block_anchor::block_anchor_replacer;
use super::boundary::trimmed_boundary_replacer;
use super::indentation::indentation_flexible_replacer;
use super::line_trimmed::line_trimmed_replacer;
use super::simple::simple_replacer;
use super::whitespace::whitespace_normalized_replacer;
use anyhow::Result;
type Replacer = fn(&str, &str) -> Vec<String>;

/// Apply replacement with multiple strategies
pub(super) fn replace(content: &str, old: &str, new: &str, replace_all: bool) -> Result<String> {
    if old == new {
        anyhow::bail!("oldString and newString must be different");
    }
    let replacers: Vec<Replacer> = vec![
        simple_replacer,
        line_trimmed_replacer,
        block_anchor_replacer,
        whitespace_normalized_replacer,
        indentation_flexible_replacer,
        trimmed_boundary_replacer,
    ];
    for replacer in replacers {
        let matches = replacer(content, old);
        for search in matches {
            if !content.contains(&search) {
                continue;
            }
            if replace_all {
                return Ok(content.replace(&search, new));
            }
            let first = content.find(&search);
            let last = content.rfind(&search);
            if first != last {
                continue; // Multiple matches, need more context
            }
            if let Some(idx) = first {
                return Ok(format!(
                    "{}{}{}",
                    &content[..idx],
                    new,
                    &content[idx + search.len()..]
                ));
            }
        }
    }
    anyhow::bail!("oldString not found in content. Provide more context or check for typos.")
}
