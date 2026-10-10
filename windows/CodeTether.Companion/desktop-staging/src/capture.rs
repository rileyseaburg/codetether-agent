//! Public selected-monitor capture boundary; never grants local consent.
use crate::{CapturedFrame, Error, Monitor};
use std::sync::atomic::AtomicBool;

/// Capture a locally selected monitor into a bounded, memory-only JPEG.
///
/// # Arguments
/// * `selected` — Snapshot chosen by the local user, never by a remote request.
/// * `cancelled` — Per-operation flag, initially false. Set on pause, stop,
///   session/display change or exit; never reset or reuse it.
///
/// # Returns
/// At most 512 KiB of JPEG with a longest edge of at most 1600 physical pixels.
/// Adapter-owned DIB, RGB and JPEG buffers are cleared when dropped; nothing
/// is persisted. This does not guarantee erasure of OS/driver buffers or
/// temporary storage inside the JPEG encoder.
///
/// # Errors
/// Returns an error on cancellation, unavailable desktop, changed selection,
/// unsupported geometry, capture failure or unsuccessful bounded encoding.
/// On non-Windows targets returns [`Error::Unsupported`]. The caller must
/// obtain local pairing/selected-monitor sharing authorization and discard the
/// result if its session/consent generation changed before upload. Checks here
/// are snapshots, not a lock against Windows session/display transitions.
///
/// # Examples
/// Illustrative host sequence (not executed):
/// ```text
/// // Only for an authorized shared monitor; the host owns cancellation.
/// let frame = capture_selected(&selected, &cancelled)?;
/// // Recheck the host's session/consent generation before using frame.jpeg().
/// ```
pub fn capture_selected(
    selected: &Monitor,
    cancelled: &AtomicBool,
) -> Result<CapturedFrame, Error> {
    #[cfg(windows)]
    return crate::platform::native::capture(selected, cancelled);
    #[cfg(not(windows))]
    {
        let _ = (selected, cancelled);
        Err(Error::Unsupported)
    }
}
