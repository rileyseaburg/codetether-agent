//! Navigation index for compressed user turns.
//!
//! [render_toc] identifies dropped turns by their original transcript indices.
//! It lists at most ten user turns, with first-line excerpts of 70 characters.
use codetether_provider_core::{Message, Role};
mod excerpt;
use excerpt::first_text_excerpt;

const MAX_ENTRIES: usize = 10;

const EXCERPT_CHARS: usize = 70;

/// Render a navigable index of user turns in a dropped prefix.
///
/// # Arguments
/// * `prefix` - Dropped messages in conversation order.
/// * `base_index` - Original transcript index of the first message.
///
/// # Returns
/// Index text, or an empty string if no user message has nonblank text.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::summary::toc::render_toc;
/// assert!(render_toc(&[], 0).is_empty());
/// ```
pub fn render_toc(prefix: &[Message], base_index: usize) -> String {
    let mut lines: Vec<String> = prefix
        .iter()
        .enumerate()
        .filter(|(_, m)| matches!(m.role, Role::User))
        .filter_map(|(i, m)| {
            first_text_excerpt(m).map(|e| format!("  [turn {}] {e}", base_index + i))
        })
        .collect();
    if lines.is_empty() {
        return String::new();
    }
    if lines.len() > MAX_ENTRIES {
        let skipped = lines.len() - MAX_ENTRIES;
        lines.truncate(MAX_ENTRIES);
        lines.push(format!("  … plus {skipped} more user turns"));
    }
    format!(
        "\n\n[DROPPED-RANGE INDEX] turns {}-{} were compressed. User turns in that range:\n{}\n\
         When the user allows history access, retrieve one via `context_browse` (action=show_turn, turn=N) or a targeted `session_recall` query.",
        base_index,
        base_index + prefix.len().saturating_sub(1),
        lines.join("\n")
    )
}

#[cfg(test)]
mod tests;
