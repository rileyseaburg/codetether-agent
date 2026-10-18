//! Clear terminal contents without querying cursor position.
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub(crate) fn clear_screen(writer: &mut impl Write) -> io::Result<()> {
    execute!(writer, Clear(ClearType::All))
}
#[cfg(test)]
#[path = "clear_tests.rs"]
mod tests;
