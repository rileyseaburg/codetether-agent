//! Executable stdio MCP fixture for subprocess approval tests.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub(super) fn create(root: &Path) -> (PathBuf, PathBuf) {
    let marker = root.join("started");
    let server = root.join("fixture-mcp");
    let script = format!(
        "#!/bin/sh\nprintf 'started\\n' >> '{}'\nread request\nid=$(printf '%s' \"$request\" | sed -n 's/.*\"id\":\\([0-9]*\\).*/\\1/p')\nprintf '{{\"jsonrpc\":\"2.0\",\"id\":%s,\"result\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{}},\"serverInfo\":{{\"name\":\"fixture\",\"version\":\"1\"}}}}}}\\n' \"$id\"\nsleep 1\n",
        marker.display()
    );
    std::fs::write(&server, script).expect("fixture script");
    let mut permissions = std::fs::metadata(&server).expect("metadata").permissions();
    permissions.set_mode(0o700);
    std::fs::set_permissions(&server, permissions).expect("executable fixture");
    (marker, server)
}
