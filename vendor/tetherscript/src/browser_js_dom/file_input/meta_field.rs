use super::*;

pub(super) fn apply(file: &mut AgentFile, key: &str, cur: &mut cursor::Cursor) -> Option<()> {
    match key {
        "name" => file.name = json_string::parse(cur)?,
        "type" => file.mime_type = json_string::parse(cur)?,
        "size" => file.size = json_number::parse(cur)?,
        "lastModified" => file.last_modified = json_number::parse(cur)?,
        _ => return None,
    }
    Some(())
}
