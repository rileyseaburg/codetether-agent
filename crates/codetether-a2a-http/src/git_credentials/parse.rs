//! Parse Git credential protocol fields from text.
use super::GitCredentialQuery;
/// Parse known non-empty fields, ignoring unknown keys.
pub fn parse_query(input: &str) -> GitCredentialQuery {
    let mut query = GitCredentialQuery::default();
    for line in input.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if value.trim().is_empty() {
            continue;
        }
        {
            match key.trim() {
                "protocol" => query.protocol = Some(value.trim().to_string()),
                "host" => query.host = Some(value.trim().to_string()),
                "path" => query.path = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }
    query
}
