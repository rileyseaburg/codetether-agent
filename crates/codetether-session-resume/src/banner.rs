//! Visible warning text for session-resume outcomes.
use crate::SessionLoadOutcome;
impl SessionLoadOutcome {
    /// Return a banner only when the user needs a visible startup warning.
    ///
    /// # Returns
    /// Warning text for failed or truncated loads; None for unremarkable startup.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_session_resume::SessionLoadOutcome::Fresh.banner().is_none());
    /// ```
    pub fn banner(&self) -> Option<String> {
        match self {
            Self::ScanFailed { reason } => {
                let msg = format!(
                    "⚠ WARNING: Your previous session could not be resumed.\n\
                     Reason: {reason}\n\
                     A new session has been started. Your old session is still on disk."
                );
                Some(msg)
            }
            Self::Loaded { dropped, .. } if *dropped > 0 => {
                let msg = format!(
                    "ℹ Session resumed with tail-cap: {dropped} older messages were not \
                     loaded to stay within the context window. Your full history is \
                     preserved on disk."
                );
                Some(msg)
            }
            _ => None,
        }
    }
}
