use crossterm::cursor;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};
use std::io::Result;
use std::io::Stdout;
use std::io::Write;

pub struct Terminal {
    x_max: u16,
    y_max: u16,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Terminal> {
        let (x_max, y_max) = crossterm::terminal::size()?;

        Ok(Terminal {
            x_max,
            y_max,
            stdout: std::io::stdout(),
        })
    }
    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        queue!(self.stdout, Clear(ClearType::All))?;
        queue!(self.stdout, cursor::MoveTo(1, 1))?;
        self.stdout
            .flush()
    }
    pub fn restore(&mut self) -> Result<()> {
        queue!(self.stdout, Clear(ClearType::All))?;
        queue!(self.stdout, cursor::MoveTo(1, 1))?;
        self.stdout
            .flush()?;
        disable_raw_mode()
    }
}

pub struct Nanoo {
    buffer: Vec<String>,
    terminal: Terminal,
}
