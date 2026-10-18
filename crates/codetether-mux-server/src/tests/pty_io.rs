//! Polling and output collection helpers for PTY persistence tests.

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::super::context::ServerContext;

pub(super) async fn wait_for_exit(context: &ServerContext) {
    for _ in 0..500 {
        if !context.programs.read(0, 0).await.unwrap().running {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    panic!("PTY did not exit within 5s");
}

pub(super) async fn read_all(connection: &mut MuxConnection, offset: &mut u64) -> Vec<u8> {
    let mut output = Vec::new();
    loop {
        let response = connection
            .request(ClientRequest::Program {
                request: ProgramRequest::Read {
                    window_id: 0,
                    offset: *offset,
                },
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

pub(super) use super::pty_file_wait::wait_for;
