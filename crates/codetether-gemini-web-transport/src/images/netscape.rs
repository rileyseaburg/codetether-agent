use super::{cookie_record::record, cookies::Jar};
use anyhow::{Result, ensure};

pub(super) fn parse(source: &str) -> Result<Jar> {
    ensure!(source.len() <= 1024 * 1024, "cookie_export_too_large");
    let mut cookies = Vec::new();
    for raw in source.trim_start_matches('\u{feff}').lines() {
        let line = raw.trim_start();
        if line.is_empty() || (line.starts_with('#') && !line.starts_with("#HttpOnly_")) {
            continue;
        }
        let fields: Vec<_> = line
            .strip_prefix("#HttpOnly_")
            .unwrap_or(line)
            .split('\t')
            .collect();
        cookies.push(record(&fields)?);
    }
    ensure!(!cookies.is_empty(), "cookie_export_empty");
    Ok(Jar(cookies))
}
