//! Translate [`RgArgs`] into a ripgrep argument vector.

use super::args::RgArgs;

/// Build the ripgrep CLI arguments for `args`.
///
/// `--` terminates flags so a pattern beginning with `-` is never parsed as a
/// flag, and search paths are appended after it.
pub(crate) fn build(args: &RgArgs) -> Vec<String> {
    let mut out = vec!["--line-number".to_string(), "--with-filename".to_string()];
    if args.fixed_strings {
        out.push("--fixed-strings".into());
    }
    if args.case_insensitive {
        out.push("--ignore-case".into());
    }
    if args.files_with_matches {
        out.push("--files-with-matches".into());
    }
    if args.hidden {
        out.push("--hidden".into());
    }
    if args.no_ignore {
        out.push("--no-ignore".into());
    }
    if let Some(context) = args.context_lines {
        out.push(format!("--context={context}"));
    }
    if let Some(max) = args.max_count {
        out.push(format!("--max-count={max}"));
    }
    for glob in &args.glob {
        out.push(format!("--glob={glob}"));
    }
    out.push("--".into());
    out.push(args.pattern.clone());
    if args.paths.is_empty() {
        out.push(".".into());
    } else {
        out.extend(args.paths.iter().cloned());
    }
    out
}
