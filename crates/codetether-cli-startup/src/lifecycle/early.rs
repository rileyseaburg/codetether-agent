//! Select native commands before environment and service initialization.
use super::Host;

pub(super) async fn run<H: Host>() -> Option<anyhow::Result<()>> {
    if !std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "windows" || arg == "bonsai")
    {
        return None;
    }
    H::native(&H::parse()).await
}
