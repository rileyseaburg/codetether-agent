//! Language-level process_run forwarding prelude.
pub const SOURCE: &str = r#"
fn process_run(command, args, stdin, timeout_ms) {
    return codetether_process.run(command, args, stdin, timeout_ms)
}
"#;

/// Prefix script source with the host-backed process_run definition.
///
/// # Arguments
/// * `source` - Original plugin source.
/// # Returns
/// Prelude followed by the original source without further modification.
/// # Examples
/// ```rust
/// let source=codetether_tetherscript_process::inject("fn main() {}".into());
/// assert!(source.ends_with("fn main() {}"));
/// assert!(source.contains("codetether_process.run"));
/// ```
pub fn inject(source: String) -> String {
    format!("{SOURCE}{source}")
}
