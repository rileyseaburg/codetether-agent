//! Access-mode status, release summary and frontend updates.
use super::Host;
use codetether_config::AccessMode;

use super::{parse, policy};

pub(super) async fn push(host: &mut impl Host, cwd: &std::path::Path, mode: AccessMode) {
    let released = (mode == AccessMode::Full).then(|| host.release_all());
    let text = format!(
        "Access mode set to `{}`.\n{}",
        parse::label(mode),
        policy::summary(cwd).await
    );
    host.push_system(with_release_note(text, released));
    host.set_status(format!("Access mode: {}", parse::label(mode)));
    host.scroll_to_bottom();
    host.clear_input();
}

fn with_release_note(text: String, released: Option<Result<usize, String>>) -> String {
    match released {
        Some(Ok(count)) if count > 0 => {
            format!("{text}\nApproved {count} paused tool request(s).")
        }
        Some(Err(error)) => format!("{text}\nCould not release paused requests: {error}"),
        Some(Ok(_)) | None => text,
    }
}
