/// Chat layout mode: Webview dashboard or classic chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatLayoutMode {
    Classic,
    #[default]
    Webview,
}

impl ChatLayoutMode {
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
