use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use ignore::WalkBuilder;

pub fn representatives(root: &Path) -> Vec<PathBuf> {
    let mut languages = HashSet::new();
    WalkBuilder::new(root)
        .hidden(false)
        .build()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
        .filter_map(|entry| select(entry.path(), &mut languages))
        .take(5)
        .collect()
}

fn select(path: &Path, languages: &mut HashSet<&'static str>) -> Option<PathBuf> {
    let text = path.to_string_lossy();
    let language = crate::lsp::detect_language_from_path(&text)?;
    languages.insert(language).then(|| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::representatives;

    #[test]
    fn selects_one_file_per_language() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("one.rs"), "").unwrap();
        std::fs::write(root.path().join("two.rs"), "").unwrap();
        std::fs::write(root.path().join("web.tsx"), "").unwrap();
        assert_eq!(representatives(root.path()).len(), 2);
    }
}
