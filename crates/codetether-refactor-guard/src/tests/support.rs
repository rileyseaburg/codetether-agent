use super::super::{lines, types::*};

pub fn file(path: &str, old: Option<&str>, new: &str, limit: usize) -> GuardFile {
    GuardFile {
        path: path.into(),
        status: if old.is_some() {
            FileStatus::Modified
        } else {
            FileStatus::Added
        },
        old_code_lines: old.map(lines::code_lines),
        new_code_lines: lines::code_lines(new),
        limit,
        wrapper_target: super::super::wrapper::target(path, new),
        old_text: old.map(str::to_string),
        new_text: new.into(),
    }
}
