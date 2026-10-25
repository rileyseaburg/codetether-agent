use super::cache_permissions::private;
use anyhow::{Result, anyhow, ensure};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

pub(super) fn read(directory: &Path, key: &str) -> Option<Vec<u8>> {
    let path = directory.join(format!("{key}.json"));
    if !private(directory, true) || !private(&path, false) {
        return None;
    }
    let file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.take(65537).read_to_end(&mut bytes).ok()?;
    (bytes.len() <= 65536).then_some(bytes)
}
pub(super) fn write(directory: &Path, key: &str, bytes: &[u8]) -> Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(directory)
        .map_err(|_| anyhow!("cookie_cache_directory_failed"))?;
    ensure!(private(directory, true), "cookie_cache_not_private");
    let mut file = tempfile::NamedTempFile::new_in(directory)
        .map_err(|_| anyhow!("cookie_cache_write_failed"))?;
    file.write_all(bytes)
        .map_err(|_| anyhow!("cookie_cache_write_failed"))?;
    file.as_file()
        .sync_all()
        .map_err(|_| anyhow!("cookie_cache_write_failed"))?;
    file.persist(directory.join(format!("{key}.json")))
        .map_err(|_| anyhow!("cookie_cache_write_failed"))?;
    Ok(())
}
