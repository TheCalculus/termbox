#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::{self, Write, stdout, Stdout};
use std::ops::Not;
use crossterm::{
    QueueableCommand,
    cursor,
    execute,
    queue,
    style,
    style::{Color, Attribute, Attributes, Print},
    terminal::{enable_raw_mode, size, Clear, ClearType},
};

#[derive(Clone)]
struct Cell {
    cchar: char,
    fg:    Color,
    bg:    Color,
    attr:  Attributes,
}

struct Update {
    cell:  Cell,
    index: usize,
}

impl Cell {
    fn default() -> Self {
        Self {
            cchar: ' ',
            fg:    Color::White,
            bg:    Color::Black,
            attr:  Attribute::Reset.into(),
        }
    }
}

struct Buffer {
    cells:   Vec<Cell>,
    updates: Vec<Update>,
    dims:    (u16, u16),
    write:   Stdout,
}

impl Buffer {
    fn default() -> Self {
        let buffsiz     = size().unwrap();
        let area: usize = (buffsiz.0 * buffsiz.1).into();

        Self {
            cells:   vec![Cell::default(); area],
            updates: Vec::new(),
            dims:    buffsiz,
            write:   stdout(),
        }
    }

    fn add_update(&mut self, cell: Cell, index: usize) {
        self.updates.push(Update { cell, index });
    }

    fn render(&mut self) -> io::Result<()> {
        self.write.queue(Clear(ClearType::All))?;
        self.write.queue(cursor::MoveTo(0, 0))?;

        let mut foreground = Color::White;
        let mut background = Color::Black;

        for update in &self.updates {
            let index = update.index;
            let cell  = &self.cells[index];

            self.write.queue(cursor::MoveTo(
                    (index as u16) % self.dims.0,
                    (index as u16) / self.dims.0,
                    ))?;
            if foreground != cell.fg {
                foreground = cell.fg;
                self.write.queue(style::SetForegroundColor(foreground))?;
            }

            if background != cell.bg {
                background = cell.bg;
                self.write.queue(style::SetBackgroundColor(background))?;
            }

            self.write.queue(Print(cell.cchar))?;
        }

        self.updates.clear();
        self.write.flush()?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut buff = Buffer::default();

    for _ in 0..5 {
        for i in 0..buff.cells.len() {
            let x = (i as u16) % buff.dims.0;
            let y = (i as u16) / buff.dims.0;

            let fg_color = match i % 3 {
                0 => Color::Red,
                1 => Color::Green,
                _ => Color::Blue,
            };

            let bg_color = Color::Black;

            let cell_char = match i % 4 {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                _ => 'D',
            };

            buff.add_update(
                Cell {
                    cchar: cell_char,
                    fg:    fg_color,
                    bg:    bg_color,
                    attr:  Attribute::Reset.into()
                }, i
                );
        }

        buff.render()?;
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(()) 
}
