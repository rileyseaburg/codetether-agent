//! Dynamic import reference scanning for module resources.

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DynamicImport {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) url: String,
}

pub(crate) fn collect(source: &str) -> Vec<DynamicImport> {
    let mut out = Vec::new();
    let mut offset = 0;
    while let Some(start) = source[offset..].find("import(").map(|hit| offset + hit) {
        let spec_start = start + "import(".len();
        let Some((url, end)) = quoted(source, spec_start) else {
            offset = spec_start;
            continue;
        };
        out.push(DynamicImport { start, end, url });
        offset = end;
    }
    out
}

fn quoted(source: &str, start: usize) -> Option<(String, usize)> {
    let rest = source[start..].trim_start();
    let ws = source[start..].len() - rest.len();
    let quote_index = start + ws;
    let quote = source.as_bytes().get(quote_index).copied()? as char;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let tail_start = quote_index + 1;
    let tail = &source[tail_start..];
    let end_quote = tail.find(quote)? + tail_start;
    let close = source[end_quote + 1..].find(')')? + end_quote + 2;
    Some((source[tail_start..end_quote].into(), close))
}
