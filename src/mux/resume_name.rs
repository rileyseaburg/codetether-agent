//! Derive a valid mux session name from a durable CodeTether session ID.
//!
//! Mux names are filesystem record stems limited to 64 characters of letters,
//! numbers, `-`, and `_`. Session IDs are UUIDs, which are longer than a useful
//! name and share a common prefix shape, so the name is built from a short
//! prefix of the ID. `resume-` marks the origin so `mux ls` shows why the
//! session exists.

/// Characters of the session ID retained in a derived mux name.
const ID_PREFIX: usize = 8;

/// Builds a mux session name for resuming `session_id`.
///
/// # Examples
///
/// ```
/// use codetether_agent::mux::resume_name::derive;
///
/// assert_eq!(derive("bd4e55f2-627e-4fa5-9ed4-a9339f6721a6"), "resume-bd4e55f2");
/// // Characters outside the mux alphabet are dropped.
/// assert_eq!(derive("ab.cd/ef!gh"), "resume-abcdefgh");
/// ```
pub fn derive(session_id: &str) -> String {
    let stem: String = session_id
        .chars()
        .filter(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_'))
        .take(ID_PREFIX)
        .collect();
    format!("resume-{stem}")
}

#[cfg(test)]
#[path = "resume_name_tests.rs"]
mod tests;
