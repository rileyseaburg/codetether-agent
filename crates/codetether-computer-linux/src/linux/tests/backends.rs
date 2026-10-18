//! Session detection and compositor-safe backend selection need no environment mutation.

use super::*;

#[test]
fn session_detection_handles_headless_x11_and_xwayland() {
    assert_eq!(Session::from_values("", "", "tty"), Session::Headless);
    assert_eq!(Session::from_values(" ", "", "wayland"), Session::Headless);
    assert_eq!(Session::from_values(":0", "", "x11"), Session::X11);
    assert_eq!(
        Session::from_values(":0", "wayland-0", ""),
        Session::Wayland
    );
    assert_eq!(Session::from_values(":0", "", "Wayland"), Session::Wayland);
}

#[test]
fn wayland_never_selects_x11_capture_backends() {
    assert!(backend::candidates(Session::Headless, "GNOME").is_empty());
    for desktop in ["", "GNOME", "KDE", "ubuntu:GNOME"] {
        let choices = backend::candidates(Session::Wayland, desktop);
        assert!(choices.contains(&Backend::Grim));
        for forbidden in [Backend::Scrot, Backend::Maim, Backend::ImageMagick] {
            assert!(!choices.contains(&forbidden));
        }
    }
    assert_eq!(
        backend::candidates(Session::X11, "KDE")[0],
        Backend::Spectacle
    );
    assert!(backend::candidates(Session::X11, "GNOME").contains(&Backend::ImageMagick));
    assert_eq!(Backend::Gnome.arguments(), &["--file"]);
    assert_eq!(Backend::Grim.arguments(), &["-t", "png"]);
    assert_eq!(Backend::ImageMagick.arguments(), &["-window", "root"]);
    assert_eq!(Backend::Maim.arguments(), &["--format=png"]);
    assert_eq!(Backend::Scrot.arguments(), &["--silent"]);
}
