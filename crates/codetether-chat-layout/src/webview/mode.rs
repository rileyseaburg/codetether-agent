/// Chat layout mode: Webview dashboard or classic chat.
/// # Examples
/// ```
/// use codetether_chat_layout::webview::ChatLayoutMode;
/// assert!(matches!(ChatLayoutMode::default(), ChatLayoutMode::Webview));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatLayoutMode {
    /// Classic scrolling chat view.
    Classic,
    /// Responsive Webview dashboard.
    #[default]
    Webview,
}

impl ChatLayoutMode {
    /// Switch between the two chat layouts.
    pub fn toggle(self) -> Self {
        match self {
            Self::Classic => Self::Webview,
            Self::Webview => Self::Classic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChatLayoutMode;

    #[test]
    fn webview_is_default_layout() {
        assert_eq!(ChatLayoutMode::default(), ChatLayoutMode::Webview);
    }
}
