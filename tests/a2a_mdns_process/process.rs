use std::fs::File;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct PeerProcess {
    child: Child,
    log: std::path::PathBuf,
}

impl PeerProcess {
    pub fn start(directory: &Path, name: &str, token: &str) -> Self {
        let log = directory.join(format!("{name}.log"));
        let stderr = File::create(&log).expect("create peer log");
        let child = Command::new(env!("CARGO_BIN_EXE_codetether"))
            .args(["spawn", "--name", name])
            .env("CODETETHER_AUTH_TOKEN", token)
            .env("CODETETHER_A2A_PEERS", "")
            .env("RUST_LOG", "codetether_agent::a2a=info")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(stderr)
            .spawn()
            .expect("spawn independent CodeTether peer");
        Self { child, log }
    }

    pub fn log(&self) -> String {
        std::fs::read_to_string(&self.log).unwrap_or_default()
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn assert_running(&mut self) {
        if let Some(status) = self.child.try_wait().expect("read peer status") {
            panic!("peer exited {status}; log:\n{}", self.log());
        }
    }
}

impl Drop for PeerProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
