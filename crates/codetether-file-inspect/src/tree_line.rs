//! tree line for filesystem inspection.
use super::*;
pub(super) fn render(
    entry: &TreeEntry,
    prefix: &str,
    is_last: bool,
    show_size: bool,
    file_count: &mut usize,
    dir_count: &mut usize,
) -> String {
    let connector = if is_last { "└── " } else { "├── " };

    let mut line = format!("{}{}", prefix, connector);

    if entry.is_dir {
        *dir_count += 1;
        line.push_str(&format!("{}/", entry.name));
    } else {
        *file_count += 1;
        if show_size {
            let size = format_size(entry.size);
            line.push_str(&format!("{} ({})", entry.name, size));
        } else {
            line.push_str(&entry.name);
        }
    }

    line
}
