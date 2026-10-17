//! Linux process ownership regression using a real shell and worker process.

use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::test]
async fn dropping_lsp_process_stops_descendant_workers() {
    let args = vec!["-c".into(), "sleep 60 & echo $!; wait".into()];
    let mut server = super::spawn("/bin/sh", &args).unwrap();
    let root = server.id().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());
    let mut line = String::new();
    tokio::time::timeout(Duration::from_secs(5), stdout.read_line(&mut line))
        .await
        .unwrap()
        .unwrap();
    let worker: i32 = line.trim().parse().unwrap();
    // Read-only syscall verifies isolation from the test runner's group.
    assert_eq!(unsafe { libc::getpgid(worker) }, root as i32);
    drop(server);
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match tokio::fs::read_to_string(format!("/proc/{worker}/stat")).await {
                Ok(stat) => {
                    let state = stat.rsplit_once(") ").unwrap().1.as_bytes()[0];
                    if state == b'Z' || state == b'X' {
                        break;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
                Err(error) => panic!("reading worker status: {error}"),
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("LSP worker survived transport eviction");
}
