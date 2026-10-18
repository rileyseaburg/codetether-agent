//! Select installed capture utilities; never use X11 capture under Wayland.

pub(super) use super::discovery::available;
#[cfg(test)]
pub(super) use super::discovery::candidates;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Backend {
    Gnome,
    Spectacle,
    Grim,
    Scrot,
    Maim,
    ImageMagick,
}

impl Backend {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Gnome => "gnome-screenshot",
            Self::Spectacle => "spectacle",
            Self::Grim => "grim",
            Self::Scrot => "scrot",
            Self::Maim => "maim",
            Self::ImageMagick => "import",
        }
    }

    pub(super) fn arguments(self) -> &'static [&'static str] {
        match self {
            Self::Gnome => &["--file"],
            Self::Spectacle => &["--background", "--nonotify", "--fullscreen", "--output"],
            Self::Grim => &["-t", "png"],
            Self::Scrot => &["--silent"],
            Self::Maim => &["--format=png"],
            Self::ImageMagick => &["-window", "root"],
        }
    }
}
