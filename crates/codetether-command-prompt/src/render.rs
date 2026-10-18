//! Drawing and redrawing the mux control prompt.

use std::io::{Write, stdout};

use anyhow::Result;
use crossterm::{
    cursor::MoveToColumn,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use unicode_width::UnicodeWidthStr;

use super::state::State;

pub(super) fn prompt(state: &State) -> Result<()> {
    let line = state.line();
    let column = 5 + UnicodeWidthStr::width(state.prefix().as_str());
    let mut output = stdout();
    queue!(
        output,
        MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        Print("mux> "),
        Print(line)
    )?;
    queue!(output, MoveToColumn(column.min(u16::MAX as usize) as u16))?;
    output.flush()?;
    Ok(())
}

pub(super) fn matches(values: &[String]) -> Result<()> {
    if values.len() > 1 {
        print!("\r\n{}\r\n", values.join("  "));
    }
    Ok(())
}
