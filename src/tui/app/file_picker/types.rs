use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePickerMode {
    Browse,
    Viewer,
}

#[derive(Debug, Clone)]
pub struct FilePreview {
    pub path: PathBuf,
    pub lines: Vec<String>,
    pub truncated: bool,
    pub binary: bool,
}

#[derive(Debug, Clone)]
pub struct FilePickerState {
    pub active: bool,
    pub workspace_dir: PathBuf,
    pub dir: PathBuf,
    pub entries: Vec<FilePickerEntry>,
    pub selected: usize,
    pub filter: String,
    pub mode: FilePickerMode,
    pub preview: Option<FilePreview>,
    pub preview_scroll: usize,
}

impl Default for FilePickerState {
    fn default() -> Self {
        Self {
            active: false,
            workspace_dir: PathBuf::new(),
            dir: PathBuf::new(),
            entries: Vec::new(),
            selected: 0,
            filter: String::new(),
            mode: FilePickerMode::Browse,
            preview: None,
            preview_scroll: 0,
        }
    }
}
