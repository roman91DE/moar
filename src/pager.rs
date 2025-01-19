use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::terminal::{Clear, ClearType};
use std::io::Result;
use std::io::Stdout;
use std::io::Write;
use std::time::Duration;

use crate::utils;

const LINE_CAP_BUF: usize = 100;

pub struct Terminal {
    x_max: u16,
    y_max: u16,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Terminal> {
        let (x_max, y_max) = size()?;

        let mut terminal = Terminal {
            x_max,
            y_max,
            stdout: std::io::stdout(),
        };

        terminal.init()?;
        Ok(terminal)
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
    pub fn redraw_terminal(&mut self, buffer: &[String]) -> Result<()> {
        let mut lines_available = self.y_max as isize; // Total lines available on the terminal screen
        queue!(self.stdout, Clear(ClearType::All))?; // Clear the terminal before redrawing
    
        for (i, line) in buffer.iter().enumerate() {
            if lines_available <= 0 {
                break; // Stop rendering if no more lines are available
            }
    
            let line_chunks = (line.len() as u16 + self.x_max - 1) / self.x_max; // Calculate how many terminal rows the line occupies
            let lines_used = std::cmp::min(line_chunks as isize, lines_available); // Ensure we don't overflow the terminal height
    
            queue!(self.stdout, MoveTo(0, i as u16))?; // Position cursor at the correct line
            queue!(self.stdout, Print(line))?; // Print the line content
    
            lines_available -= lines_used; // Update available lines count
        }
    
        self.stdout.flush() // Ensure the output is rendered
    }
    
}

pub struct Args {
    pub filepath: Option<String>,
}

pub struct Moar {
    buffer: Vec<String>,
    terminal: Terminal,
    kill: bool,
    start_line: usize,
}

impl Moar {
    pub fn new(args: Args) -> Result<Moar> {
        let buffer: Vec<String> = match args.filepath {
            Some(s) => utils::read_file_to_vec(&s).unwrap_or(Vec::with_capacity(LINE_CAP_BUF)),
            None => Vec::with_capacity(LINE_CAP_BUF),
        };

        let terminal = Terminal::new()?;

        Ok(Moar {
            buffer,
            terminal,
            kill: false,
            start_line: 0,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.kill {
            // Redraw terminal with the current buffer view
            let end_line = (self.start_line
                + self
                    .terminal
                    .y_max as usize)
                .min(
                    self.buffer
                        .len(),
                );
            self.terminal
                .redraw_terminal(&self.buffer[self.start_line..end_line])?;

            // Listen for user input
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            self.kill = true; // Quit on 'q'
                        }
                        KeyCode::Down  => {
                            if self.start_line
                                + (self
                                    .terminal
                                    .y_max as usize)
                                < self
                                    .buffer
                                    .len()
                            {
                                self.start_line += 1; // Scroll down
                            }
                        }
                        KeyCode::Up => {
                            if self.start_line > 0 {
                                self.start_line -= 1; // Scroll up
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        self.terminal
            .restore() // Restore terminal settings before exiting
    }
}
