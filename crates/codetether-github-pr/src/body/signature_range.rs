//! PR body signature range.
use super::*;
pub(super) fn signature_range(body: &str) -> Option<(usize, usize)> {
    let start = body.find(START)?;
    let end = body[start..].find(END)?;
    Some((start, start + end + END.len()))
}
