use std::{io, iter::Peekable, slice::Iter};
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};
/// the help command brings up a TUI
/// interactive help interface, to help
/// find scripts.
pub fn help(root: &str, mut args: Peekable<Iter<String>>) -> Result<String, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.hide_cursor()?;
    loop {
        terminal.draw(|mut f| {
            Block::default()
                .title("Interactive Help")
                .borders(Borders::ALL)
                .render(&mut f, size);
        })?;
    }
    Ok(String::from(""))
}
