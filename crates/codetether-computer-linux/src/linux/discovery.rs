//! Discover installed utilities compatible with the active display protocol.

use super::{backend::Backend, session::Session};
use std::path::PathBuf;

pub(super) fn candidates(session: Session, desktop: &str) -> Vec<Backend> {
    use Backend::*;
    let mut choices = match session {
        Session::Headless => vec![],
        Session::Wayland => vec![Gnome, Spectacle, Grim],
        Session::X11 => vec![Gnome, Spectacle, Scrot, Maim, ImageMagick],
    };
    if desktop
        .to_ascii_lowercase()
        .split(':')
        .any(|part| part == "kde")
        && !choices.is_empty()
    {
        choices.swap(0, 1);
    }
    choices
}

pub(super) fn available(session: Session) -> Vec<(Backend, PathBuf)> {
    candidates(
        session,
        &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default(),
    )
    .into_iter()
    .filter_map(|backend| {
        which::which(backend.name())
            .ok()
            .map(|path| (backend, path))
    })
    .collect()
}
