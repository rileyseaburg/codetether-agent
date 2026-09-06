//! Shared mock-only fixtures for serial and parallel recording tests.

mod assertions;
mod provider;
mod tool;

pub(super) use assertions::assert_recorded;
pub(super) use provider::RejectProvider;
pub(super) use tool::ImageTool;

use crate::session::Session;
use std::path::Path;

pub(super) async fn session(cwd: &Path) -> Session {
    let mut session = Session::new().await.expect("mock session");
    session.metadata.directory = Some(cwd.to_path_buf());
    session.metadata.model = Some("image-recording-mock/test".into());
    session.metadata.rlm.mode = "off".into();
    session
}
