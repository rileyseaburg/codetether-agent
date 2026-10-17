//! Command-line capture for allocator crash reports.

use std::ffi::OsString;

/// Return the current command line without panicking on invalid Unicode.
pub(crate) fn command_line() -> String {
    command_line_from(std::env::args_os())
}

/// Join command-line arguments using lossy UTF-8 conversion.
pub(crate) fn command_line_from<I>(args: I) -> String
where
    I: IntoIterator<Item = OsString>,
{
    args.into_iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}
