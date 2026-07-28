use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::capability::Capability;
use crate::fs_cap::FsAuthority;
use crate::value::{Runtime, Value};

struct NoopRuntime;

impl Runtime for NoopRuntime {
    fn invoke(&mut self, _callee: &Value, _args: &[Value]) -> Result<Value, String> {
        Err("unexpected callback".into())
    }
}

#[test]
fn load_reads_files_through_fs_capability() {
    let root = temp_root();
    fs::create_dir_all(root.join("dist/assets")).unwrap();
    fs::write(root.join("dist/index.html"), "home").unwrap();
    fs::write(root.join("dist/assets/app.js"), "console.log(1)").unwrap();

    let fs_auth = FsAuthority::new(&root);
    let capability = Capability::new_root("fs", fs_auth);
    let mut runtime = NoopRuntime;
    let site = super::load(&mut runtime, &capability, "dist").unwrap();

    let body = String::from_utf8_lossy(site.route("/").unwrap().bytes("GET", false));
    assert!(body.ends_with("home"));
    assert!(site.route("/assets/app.js").is_some());
    fs::remove_dir_all(root).unwrap();
}

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("tetherscript-http-static-{nanos}"))
}
