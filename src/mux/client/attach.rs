//! Interactive attachment to a persistent mux server.

use std::io::{self, Write};

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::mux::protocol::ClientRequest;
use crate::mux::registry::MuxRecord;

use super::connection::MuxConnection;
use super::parse::ParsedCommand;

pub(in crate::mux) async fn attach(record: &MuxRecord) -> Result<()> {
    let mut connection = MuxConnection::connect(record).await?;
    super::render::help();
    let response = connection.request(ClientRequest::Snapshot).await?;
    let mut state = None;
    super::state::update(&mut state, &response);
    super::render::response(&response);
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        print!("mux> ");
        io::stdout().flush()?;
        let Some(line) = lines.next_line().await? else {
            break;
        };
        match super::parse::parse(&line) {
            ParsedCommand::Help => super::render::help(),
            ParsedCommand::Invalid(message) => eprintln!("mux: {message}"),
            ParsedCommand::Exec(command) => {
                super::exec::run(&command, super::state::workspace(&state)?).await?;
            }
            ParsedCommand::Request(request) => {
                let response = connection.request(request).await?;
                super::state::update(&mut state, &response);
                if super::render::response(&response) {
                    return Ok(());
                }
            }
        }
    }
    let _ = connection.request(ClientRequest::Detach).await;
    Ok(())
}
