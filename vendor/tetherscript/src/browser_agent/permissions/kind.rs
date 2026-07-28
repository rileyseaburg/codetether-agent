//! Supported browser permission names.

/// Browser permission names tracked by the agent emulator.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::permissions::BrowserPermission;
///
/// assert_eq!(BrowserPermission::Geolocation.name(), "geolocation");
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrowserPermission {
    /// Location access through `navigator.geolocation`.
    Geolocation,
    /// Notification permission metadata.
    Notifications,
    /// Clipboard read access.
    ClipboardRead,
    /// Clipboard write access.
    ClipboardWrite,
    /// Camera permission metadata.
    Camera,
    /// Microphone permission metadata.
    Microphone,
}

pub(crate) const ALL_PERMISSIONS: [BrowserPermission; 6] = [
    BrowserPermission::Geolocation,
    BrowserPermission::Notifications,
    BrowserPermission::ClipboardRead,
    BrowserPermission::ClipboardWrite,
    BrowserPermission::Camera,
    BrowserPermission::Microphone,
];

impl BrowserPermission {
    /// Return the browser-facing permission name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Geolocation => "geolocation",
            Self::Notifications => "notifications",
            Self::ClipboardRead => "clipboard-read",
            Self::ClipboardWrite => "clipboard-write",
            Self::Camera => "camera",
            Self::Microphone => "microphone",
        }
    }
}
