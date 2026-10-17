//! Detect graphical sessions without interpreting an XWayland root as a desktop.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Session {
    Headless,
    X11,
    Wayland,
}

impl Session {
    pub(super) fn detect() -> Self {
        Self::from_values(
            &std::env::var("DISPLAY").unwrap_or_default(),
            &std::env::var("WAYLAND_DISPLAY").unwrap_or_default(),
            &std::env::var("XDG_SESSION_TYPE").unwrap_or_default(),
        )
    }

    pub(super) fn from_values(display: &str, wayland: &str, kind: &str) -> Self {
        if !wayland.trim().is_empty() {
            Self::Wayland
        } else if display.trim().is_empty() {
            Self::Headless
        } else if kind.eq_ignore_ascii_case("wayland") {
            Self::Wayland
        } else {
            Self::X11
        }
    }

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Headless => "headless",
            Self::X11 => "x11",
            Self::Wayland => "wayland",
        }
    }
}
