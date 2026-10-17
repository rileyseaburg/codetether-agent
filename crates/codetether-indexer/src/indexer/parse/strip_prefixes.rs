//! strip prefixes for codebase indexing.

pub(in crate::indexer) fn strip_prefixes<'a>(mut input: &'a str, prefixes: &[&str]) -> &'a str {
    loop {
        let mut matched = false;
        for prefix in prefixes {
            if let Some(rest) = input.strip_prefix(prefix) {
                input = rest.trim_start();
                matched = true;
                break;
            }
        }

        if !matched {
            return input;
        }
    }
}
