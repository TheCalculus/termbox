#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
//  error::Error,
    io::{self, Write, stdout, Stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    QueueableCommand,
    cursor, execute, queue, style,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{Color, Attribute, Attributes, Print},
    terminal::{enable_raw_mode, disable_raw_mode, size, Clear, ClearType},
};

use rand::Rng;

#[derive(Clone)]
struct Cell {
    ch:    char,
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
        Self { ch: ' ',          fg:   Color::White, 
               bg: Color::Black, attr: Attribute::Reset.into() }
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

    fn update_cell(&mut self, cell: Cell, index: usize) {
        self.updates.push(Update { cell, index });
    }

    fn update_contiguous_cells(&mut self, index: usize, chars: Vec<char>, fg: Color, bg: Color) {
        // TODO: automatic wrap-to-next-line magic
        for (i, &ch) in chars.iter().enumerate() {
            self.update_cell(Cell { ch, fg,
                bg, attr: Attribute::Reset.into() }, index + i);
        }
    }

    fn render(&mut self) -> io::Result<()> {
        if self.updates.len() == 0 { return Ok(()); }

        for update in self.updates.iter() {
            let x = (update.index as u16) / self.dims.0;
            let y = (update.index as u16) % self.dims.0;

            self.write.queue(cursor::MoveTo(y, x));

            if update.cell.bg != Color::Black {
                self.write
                    .queue(style::SetBackgroundColor(update.cell.bg))?;
            }

            if update.cell.fg != Color::White {
                self.write
                    .queue(style::SetForegroundColor(update.cell.fg))?; 
            }

            self.write.queue(Print(update.cell.ch))?; 
        }

        self.updates.clear();
        self.write.flush()?;

        Ok(())
    }
}

#[derive(PartialEq)]
enum CursorMode {
    Normal,
    Insert,
    Visual,
}

struct Client {
    cursor:  (u16, u16),
    mode:    CursorMode,
}

fn purge(buff: &mut Buffer) -> io::Result<()> {
    buff.write.queue(Clear(ClearType::Purge))?;
    buff.write.flush()?;
    Ok(())
}

const TARGET_FRAMETIME: Duration = Duration::from_millis(33);

fn main() -> io::Result<()> {
    enable_raw_mode()?;
   
    let mut buff = Buffer::default();
    let mut prev = Instant::now();

    let mut client = Client { cursor: (0, 0), mode: CursorMode::Visual };

    purge(&mut buff)?;

    let mut running = true;
    while running {
        let curr = Instant::now();
        let delt = curr.duration_since(prev);

        if delt < TARGET_FRAMETIME {
            sleep(TARGET_FRAMETIME - delt);
            continue;
        }

        prev = curr;

        while poll(Duration::ZERO)? {
            // handle only key events
            if let Ok(Event::Key(event)) = read() {
                match event.code {
                    KeyCode::Char('i') => {
                        if client.mode == CursorMode::Normal { 
                            client.mode = CursorMode::Insert
                        }
                    },
                    KeyCode::Char('v') => {
                        if client.mode == CursorMode::Normal { 
                            client.mode = CursorMode::Visual
                        }
                    },
                    KeyCode::Char('j') => client.cursor.1 += 1,
                    KeyCode::Char('k') => client.cursor.1 -= 1,
                    KeyCode::Char('l') => client.cursor.0 += 1,
                    KeyCode::Char('h') => client.cursor.0 -= 1,
                    KeyCode::Char(':') => running     = false,
                    KeyCode::Esc       => client.mode = CursorMode::Normal,
                    _ => {},
                }
            }
        }

        buff.update_cell(Cell { ch: 'e', fg: Color::Red, bg: Color::Black, attr: Attribute::Reset.into() }, rand::thread_rng().gen_range(0..buff.cells.len()));
        buff.update_contiguous_cells(0, rand::thread_rng().gen_range(100..buff.cells.len()).to_string().as_str().chars().collect(), Color::DarkRed, Color::Black);
        buff.render()?;
    }

    purge(&mut buff)?;
    disable_raw_mode()?;
    Ok(()) 
}
