//! OpenSSL process discovery and launch.

use std::io;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub(crate) fn spawn(domain: &str, port: u16) -> io::Result<Child> {
    let mut command = Command::new(binary());
    command
        .arg("s_client")
        .arg("-quiet")
        .arg("-servername")
        .arg(domain)
        .arg("-connect")
        .arg(format!("{}:{}", domain, port))
        .arg("-verify_return_error")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command.spawn().map_err(spawn_error)
}

fn binary() -> String {
    candidates()
        .into_iter()
        .find(|path| *path == "openssl" || Path::new(path).exists())
        .unwrap_or("openssl")
        .into()
}

fn candidates() -> Vec<&'static str> {
    vec![
        r"C:\Program Files\Git\usr\bin\openssl.exe",
        r"C:\Program Files\Git\mingw64\bin\openssl.exe",
        r"C:\OpenSSL-Win64\bin\openssl.exe",
        "openssl",
    ]
}

fn spawn_error(error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("failed to spawn `openssl s_client`; install OpenSSL or use http:// reverse proxy: {error}"),
    )
}
