//! Token file parsing.
use super::read_token;

#[tokio::test]
async fn trims_surrounding_whitespace() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), "  s.fixture-token\n").unwrap();
    assert_eq!(read_token(file.path()).await.unwrap(), "s.fixture-token");
}

#[tokio::test]
async fn empty_file_is_an_error() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), " \n").unwrap();
    let error = read_token(file.path()).await.unwrap_err().to_string();
    assert!(error.contains("is empty"), "{error}");
}

#[tokio::test]
async fn missing_file_is_an_error_without_contents() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("absent-token");
    let error = read_token(&path).await.unwrap_err().to_string();
    assert!(error.contains("Failed to read Vault token file"), "{error}");
}
