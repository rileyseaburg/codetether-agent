//! Extract a file path token from a clicked chat line.
//!
//! Agents reference files inline (e.g. `src/main.rs:42`, `./lib/foo.ts`,
//! backtick-quoted paths). This scans the plain text of a clicked line for
//! the token nearest the click column that resolves to a real file under the
//! workspace, returning its absolute path.

use std::path::{Path, PathBuf};

/// Finds a file path in `line` near `click_col`, resolved against `cwd`.
///
/// Tokens are split on whitespace and common punctuation; a trailing
/// `:line[:col]` suffix is stripped. The token closest to the click column
/// that exists on disk wins.
pub fn path_at(line: &str, click_col: usize, cwd: &Path) -> Option<PathBuf> {
    let mut best: Option<(usize, PathBuf)> = None;
    let mut byte = 0usize;
    for raw in line.split(|c: char| {
        c.is_whitespace() || matches!(c, '`' | '"' | '\'' | '(' | ')' | '[' | ']' | ',')
    }) {
        let start = byte;
        byte += raw.len() + 1;
        if raw.is_empty() {
            continue;
        }
        let cleaned = strip_line_suffix(raw.trim_matches(|c: char| matches!(c, '.' | ':' | ';')));
        if !looks_like_path(cleaned) {
            continue;
        }
        let abs = resolve(cwd, cleaned);
        if !abs.is_file() {
            continue;
        }
        let dist = click_col.abs_diff(start);
        if best.as_ref().map(|(d, _)| dist < *d).unwrap_or(true) {
            best = Some((dist, abs));
        }
    }
    best.map(|(_, p)| p)
}

/// Strips a trailing `:line` or `:line:col` location suffix.
fn strip_line_suffix(tok: &str) -> &str {
    match tok.split_once(':') {
        Some((head, tail)) if tail.chars().next().is_some_and(|c| c.is_ascii_digit()) => head,
        _ => tok,
    }
}

/// Heuristic: a token is path-like if it has a separator or file extension.
fn looks_like_path(tok: &str) -> bool {
    !tok.is_empty() && (tok.contains('/') || tok.contains('.'))
}

fn resolve(cwd: &Path, tok: &str) -> PathBuf {
    let p = Path::new(tok);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}
