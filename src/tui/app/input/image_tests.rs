#[cfg(test)]
mod tests {
    use std::fs;

    use crate::tui::app::input::attach_image_file;

    fn temp_file(name: &str, bytes: &[u8]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(dir.path().join(name), bytes).expect("write image");
        dir
    }

    #[test]
    fn attaches_uppercase_extension_as_data_url() {
        let dir = temp_file("PHOTO.PNG", b"png bytes");
        let image = attach_image_file(&dir.path().join("PHOTO.PNG")).expect("image");
        assert_eq!(image.mime_type.as_deref(), Some("image/png"));
        assert!(image.data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn rejects_missing_directory_and_unknown_extension() {
        let missing = attach_image_file(std::path::Path::new("/definitely/missing.png"));
        assert!(missing.unwrap_err().contains("File not found"));
        let dir = temp_file("note.txt", b"not image");
        let err = attach_image_file(&dir.path().join("note.txt")).unwrap_err();
        assert!(err.contains("Unsupported image format"));
    }
}
