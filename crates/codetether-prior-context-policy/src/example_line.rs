//! Detection of pasted list/transcript examples.

pub(super) fn matches(input: &str) -> bool {
    let input = input.trim_start();
    ["> ", "| ", "│"]
        .iter()
        .any(|prefix| input.starts_with(prefix))
}

/// Return whether a line opens an unquoted example block.
pub(super) fn starts_block(input: &str) -> bool {
    let input = input.trim().to_lowercase();
    ["example:", "examples:", "for example:"]
        .iter()
        .any(|prefix| input.starts_with(prefix))
}
