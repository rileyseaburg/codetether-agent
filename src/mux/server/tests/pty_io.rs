//! Polling and output collection helpers for PTY persistence tests.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::super::context::ServerContext;

pub(super) async fn wait_for(path: &std::path::Path) {
    for _ in 0..100 {
        if path.exists() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    panic!("detached PTY did not write the proof file");
}

pub(super) async fn wait_for_exit(context: &ServerContext) {
    for _ in 0..100 {
        if !context.programs.read(0, 0).unwrap().running {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    panic!("PTY did not exit");
}

pub(super) async fn read_all(connection: &mut MuxConnection, offset: &mut u64) -> Vec<u8> {
    let mut output = Vec::new();
    loop {
        let response = connection
            .request(ClientRequest::ReadProgram {
                window_id: 0,
                offset: *offset,
            })
            .await
            .unwrap();
        let ServerResponse::ProgramOutput {
            data,
            next_offset,
            running,
        } = response
        else {
            panic!()
        };
        let drained = data.is_empty();
        output.extend(data);
        *offset = next_offset;
        if !running && drained {
            return output;
        }
    }
}
