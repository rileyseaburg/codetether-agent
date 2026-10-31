//! RAII hold keeps the consumer occupied while tests enqueue work.
use std::{path::Path, sync::mpsc, thread::JoinHandle, time::Duration};

pub struct Hold {
    release: Option<mpsc::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

pub fn hold(path: &Path) -> Hold {
    let path = path.to_path_buf();
    let (started, ready) = mpsc::channel();
    let (release, wait) = mpsc::channel();
    let thread = std::thread::spawn(move || {
        codetether_session_store::coordinator::write(&path, move |_| {
            started.send(())?;
            wait.recv_timeout(Duration::from_secs(10))?;
            Ok(())
        })
        .unwrap();
    });
    ready.recv_timeout(Duration::from_secs(10)).unwrap();
    Hold {
        release: Some(release),
        thread: Some(thread),
    }
}

impl Drop for Hold {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            let _ = release.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
