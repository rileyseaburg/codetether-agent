//! Zero-Rust-dependency TLS stream used by ProviderAuthority.
//!
//! This module intentionally avoids Rust TLS crates. It delegates TLS record
//! handling and certificate verification to the platform OpenSSL executable
//! (`openssl s_client`). That keeps Cargo dependency count at zero while still
//! using the system TLS implementation and CA bundle.
//!
//! The public surface is intentionally tiny: enough for HTTPS POST over an
//! already-connected TCP target.

use std::io::{self, Read, Write};
#[path = "tls_openssl.rs"]
mod openssl;

use std::process::{Child, ChildStdin, ChildStdout};

pub struct TlsConnector;

impl TlsConnector {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }

    pub fn connect(&self, domain: &str, port: u16) -> io::Result<TlsStream> {
        TlsStream::connect(domain, port)
    }
}

pub struct TlsStream {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl TlsStream {
    fn connect(domain: &str, port: u16) -> io::Result<Self> {
        let mut child = openssl::spawn(domain, port)?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("openssl stdin unavailable"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("openssl stdout unavailable"))?;

        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdout.read(buf)
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdin.flush()
    }
}

impl Drop for TlsStream {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
