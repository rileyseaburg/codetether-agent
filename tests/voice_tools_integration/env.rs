use std::sync::OnceLock;

use tokio::sync::Mutex;

pub fn voice_env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    pub fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.prev {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}
