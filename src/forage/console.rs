//! Shared quiet-mode state for CLI and TUI forage reporting.
pub use codetether_forage_reporting::console::{is_quiet, set_quiet};
#[doc(hidden)]
pub use codetether_forage_reporting::forage_println as print_line;

/// Print through the shared forage quiet-mode gate.
#[macro_export]
macro_rules! forage_println {
    ($($arg:tt)*) => {
        $crate::forage::console::print_line!($($arg)*);
    };
}
