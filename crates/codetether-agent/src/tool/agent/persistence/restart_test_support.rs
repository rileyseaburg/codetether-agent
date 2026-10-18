use crate::session::Session;
use crate::tool::agent::store::AgentEntry;

static TEST_ENV: parking_lot::Mutex<()> = parking_lot::Mutex::new(());

pub(in crate::tool::agent) struct EnvGuard {
    previous: Option<std::ffi::OsString>,
    _lock: parking_lot::MutexGuard<'static, ()>,
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match self.previous.take() {
                Some(value) => std::env::set_var("CODETETHER_DATA_DIR", value),
                None => std::env::remove_var("CODETETHER_DATA_DIR"),
            }
        }
    }
}

pub(in crate::tool::agent) fn isolate() -> (tempfile::TempDir, EnvGuard) {
    let lock = TEST_ENV.lock();
    let temp = tempfile::tempdir().unwrap();
    let guard = EnvGuard {
        previous: std::env::var_os("CODETETHER_DATA_DIR"),
        _lock: lock,
    };
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    (temp, guard)
}

pub(super) fn entry(name: &str, session: Session, owner: &str) -> AgentEntry {
    AgentEntry {
        name: name.into(),
        instructions: "recover me".into(),
        session,
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 0,
        model_id: Some("test/model".into()),
    }
}
