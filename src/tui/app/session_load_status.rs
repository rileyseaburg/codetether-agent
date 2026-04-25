use crate::session::Session;

pub fn load_status_with_original(
    session: &Session,
    dropped: usize,
    file_bytes: u64,
    original_id: Option<&str>,
) -> String {
    let label = session.title.clone().unwrap_or_else(|| session.id.clone());
    if dropped == 0 {
        return format!("Loaded session {label}");
    }
    let mb = file_bytes as f64 / (1024.0 * 1024.0);
    format!(
        "Loaded large session {label}: showing last {} entries, dropped {dropped} ({mb:.1} MiB); original {} preserved",
        session.messages.len(),
        original_id.unwrap_or("unknown")
    )
}
