#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::{self, Write, stdout, Stdout};
use crossterm::{
    QueueableCommand,
    cursor, execute, queue,
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

        for update in self.updates.iter() {
            if 
                update.cell.bg != Color::Black {
                    self.write.queue(style::SetBackgroundColor(update.cell.bg))?; 
                }
            else if
                update.cell.fg != Color::White {
                    self.write.queue(style::SetForegroundColor(update.cell.fg))?; 
                }

            self.write.queue(Print(update.cell.cchar))?; 
        }

        self.updates.clear();
        self.write.flush()?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut buff = Buffer::default();

    for i in 0..buff.cells.len() {
        buff.add_update(Cell {
            cchar: ['A', 'B', 'C', 'D'][i % 4],
            fg:    [Color::Red, Color::Green, Color::Blue][i % 3],
            bg:    Color::Black,
            attr:  Attribute::Reset.into()
        }, i);
    }

    buff.render()?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(()) 
}
