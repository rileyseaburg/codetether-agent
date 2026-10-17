//! Safe writable runtime locations for sandboxed commands.

use super::{PathRule, rule, sys};
use std::path::Path;

pub(super) fn rules(writable: u64) -> Result<Vec<PathRule>, &'static str> {
    Ok(vec![
        rule(Path::new("/dev/null"), sys::FILE_READ_WRITE_ACCESS)?,
        rule(
            &std::env::temp_dir(),
            sys::READ_ACCESS | writable,
        )?,
    ])
}