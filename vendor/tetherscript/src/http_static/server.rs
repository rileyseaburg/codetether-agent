//! Accept loop and worker-pool dispatch for the static fast path.

use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use super::site::Site;

const STATIC_WORKERS: usize = 64;

/// Bind the TCP listener and dispatch accepted sockets to workers.
pub(crate) fn serve(port: u16, site: Site) -> Result<(), String> {
    let listener = TcpListener::bind(("0.0.0.0", port))
        .map_err(|e| format!("http_serve_static: bind to 0.0.0.0:{} failed: {}", port, e))?;
    let site = Arc::new(site);
    let (tx, rx) = mpsc::channel::<TcpStream>();
    let rx = Arc::new(Mutex::new(rx));
    for _ in 0..STATIC_WORKERS {
        spawn_worker(rx.clone(), site.clone());
    }
    eprintln!(
        "tetherscript http static: listening on http://0.0.0.0:{} with {} workers",
        port, STATIC_WORKERS
    );
    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                if tx.send(stream).is_err() {
                    return Err("http_serve_static: worker queue closed".into());
                }
            }
            Err(e) => eprintln!("tetherscript http static: accept error: {}", e),
        }
    }
    Ok(())
}

fn spawn_worker(rx: Arc<Mutex<mpsc::Receiver<TcpStream>>>, site: Arc<Site>) {
    thread::spawn(move || loop {
        let stream = match rx.lock().expect("static worker receiver poisoned").recv() {
            Ok(stream) => stream,
            Err(_) => return,
        };
        if let Err(e) = super::worker::handle(stream, &site) {
            eprintln!("tetherscript http static: {}", e);
        }
    });
}
