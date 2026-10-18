//! short n in shell hook-bypass detection.

/// Commit's `-n` is `--no-verify`, alone or inside a short-flag group (`-an`).
/// Stop at `-m`/`-F`/`-C`, whose value may itself contain an `n`.
pub(super) fn short_n(arg: &str) -> bool {
    let Some(flags) = arg.strip_prefix('-') else {
        return false;
    };
    if flags.is_empty() || flags.starts_with('-') {
        return false;
    }
    for flag in flags.chars() {
        match flag {
            'n' => return true,
            'm' | 'F' | 'C' | 'c' | 't' | 'S' | 'u' => return false,
            c if c.is_ascii_alphabetic() => continue,
            _ => return false,
        }
    }
    false
}
