//! Safe writable device rules for sandboxed commands.

use super::{PathRule, rule, sys};
use std::path::Path;

pub(super) fn rules() -> Result<Vec<PathRule>, &'static str> {
    Ok(vec![rule(
        Path::new("/dev/null"),
        sys::FILE_READ_WRITE_ACCESS,
    )?])
}
