/// Chat layout mode: Classic (default) or Webview (IDE-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChatLayoutMode {
    #[default]
    Classic,
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
