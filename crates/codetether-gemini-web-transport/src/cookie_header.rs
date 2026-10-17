//! Gemini Web cookie header.
use crate::*;
impl GeminiWebSession<'_> {
    /// Convert a Netscape cookies.txt (7 tab-separated columns) into a
    /// `Cookie: name=value; ...` header string.
    ///
    /// Standard Netscape format columns:
    ///   0: domain  1: flag  2: path  3: secure  4: expiration  5: name  6: value
    ///
    /// If only 2 columns are present (simple `name\tvalue` format), those are
    /// used directly so the function remains backward-compatible.
    pub fn cookie_header(&self) -> String {
        self.cookies
            .lines()
            .filter_map(|line| {
                let t = line.trim();
                if t.is_empty() {
                    return None;
                }
                // "#HttpOnly_" prefix marks HttpOnly cookies in Netscape format —
                // strip it so the cookie is still included.  Pure comment lines
                // (e.g. the header block starting with "# Netscape") are skipped.
                let line = if let Some(rest) = t.strip_prefix("#HttpOnly_") {
                    rest
                } else if t.starts_with('#') {
                    return None;
                } else {
                    t
                };
                Some(line)
            })
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                let (name, value) = if parts.len() >= 7 {
                    // Standard Netscape format
                    (parts[5].trim(), parts[6].trim())
                } else if parts.len() >= 2 {
                    // Simple name\tvalue format (backward compat)
                    (parts[0].trim(), parts[1].trim())
                } else {
                    return None;
                };
                if name.is_empty() {
                    return None;
                }
                Some(format!("{name}={value}"))
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}
