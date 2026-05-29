use std::path::Path;

use super::scan_entry::push_entry;
use super::types::FilePickerEntry;

pub fn scan_directory(dir: &Path) -> Vec<FilePickerEntry> {
    let mut entries = parent_entry(dir).into_iter().collect::<Vec<_>>();
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return entries;
    };
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for entry in read_dir.flatten() {
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        push_entry(entry.path(), is_dir, &mut dirs, &mut files);
    }
    dirs.sort_by(|a, b| a.name.cmp(&b.name));
    files.sort_by(|a, b| a.name.cmp(&b.name));
    entries.extend(dirs);
    entries.extend(files);
    entries
}

fn parent_entry(dir: &Path) -> Option<FilePickerEntry> {
    dir.parent().map(|path| FilePickerEntry {
        path: path.to_path_buf(),
        is_dir: true,
        name: "../".to_string(),
    })
}
