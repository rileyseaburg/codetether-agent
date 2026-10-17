//! Legacy executor parse line numbered output.
use super::*;

/// Parse numbered lines, accepting an optional L prefix.
///
/// # Arguments
/// * `output` - Text containing line-number and content pairs.
/// # Returns
/// Successfully parsed lines in their original order.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::legacy_support::parse_line_numbered_output("L12:text")[0].0, 12);
/// ```
pub fn parse_line_numbered_output(output: &str) -> Vec<(usize, String)> {
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let (line_no, text) = trimmed.split_once(':')?;
            let number = line_no
                .trim()
                .trim_start_matches('L')
                .parse::<usize>()
                .ok()?;
            Some((number, text.trim_end_matches('\r').to_string()))
        })
        .collect()
}
