//! Compact PTY request fixtures for network mux tests.

use crate::mux::protocol::{ClientRequest, ProgramRequest};

pub(super) fn start(command: &str) -> ClientRequest {
    ClientRequest::Program {
        request: ProgramRequest::Start {
            window_id: 0,
            command: command.into(),
            columns: 80,
            rows: 24,
        },
    }
}

pub(super) fn attach(columns: u16, rows: u16) -> ClientRequest {
    ClientRequest::Program {
        request: ProgramRequest::Attach {
            window_id: 0,
            columns,
            rows,
        },
    }
}

pub(super) fn input(data: &[u8]) -> ClientRequest {
    ClientRequest::Program {
        request: ProgramRequest::Input {
            window_id: 0,
            data: data.to_vec(),
        },
    }
}

pub(super) fn read(offset: u64) -> ClientRequest {
    ClientRequest::Program {
        request: ProgramRequest::Read {
            window_id: 0,
            offset,
        },
    }
}
