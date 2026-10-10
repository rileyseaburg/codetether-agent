/// Redacted native desktop failures; never includes screen contents or handles.
///
/// ```
/// use codetether_companion_desktop::Error;
/// assert!(matches!(Error::Unavailable, Error::Unavailable));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Native Windows APIs are unavailable on this target.
    Unsupported,
    /// Session/desktop is inactive, inaccessible, or cannot be verified.
    Unavailable,
    /// A monitor has empty, overflowing, or excessive pixel bounds.
    Geometry,
    /// Monitor discovery failed or the locally selected monitor changed.
    Selection,
    /// The caller cancelled this capture; its token must never be reset.
    Cancelled,
    /// Native capture or GDI resource setup failed.
    Capture,
    /// The JPEG encoder failed without exceeding the bounded output size.
    Encoding,
    /// Every permitted JPEG quality exceeded the 512-KiB upload limit.
    FrameTooLarge,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unsupported => "Windows desktop APIs unavailable",
            Self::Unavailable => "Interactive desktop unavailable",
            Self::Geometry => "Unsupported monitor geometry",
            Self::Selection => "Selected monitor unavailable or changed",
            Self::Cancelled => "Capture cancelled",
            Self::Capture => "Selected-monitor capture failed",
            Self::Encoding => "Screenshot encoding failed",
            Self::FrameTooLarge => "Screenshot exceeds the upload limit",
        })
    }
}
impl std::error::Error for Error {}
