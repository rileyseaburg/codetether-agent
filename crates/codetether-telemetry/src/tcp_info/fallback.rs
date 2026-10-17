//! Platform-specific TCP observation.

use super::TcpInfoSnapshot;

/// Non-Linux platforms expose no `TCP_INFO`; `fd` is an opaque integer here.
#[cfg(not(target_os = "linux"))]
pub fn sample(_fd: i64) -> Option<TcpInfoSnapshot> {
    None
}
