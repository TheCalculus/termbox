#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::{self, Write, stdout};
use crossterm::{
    cursor,
    execute,
    terminal::{enable_raw_mode, size, Clear, ClearType},
    style::{Color, Attribute, Attributes}
};

struct Cell {
    character:  String,
    foreground: Color,
    background: Color,
    attributes: Attributes,
}

impl Cell {
    fn default() -> Self {
        return Cell {
            character: "".to_string(),
            foreground: Color::White,
            background: Color::Black,
            attributes: Attribute::Reset.into(),
        }
    }
}

struct Buffer {
    content:    Vec<Cell>,
    dimensions: (u16, u16),
}

impl Buffer {
    fn default() -> Self {
        let buffsiz     = size().unwrap();
        let area: usize = (buffsiz.0 * buffsiz.1).into();

        return Buffer {
            dimensions: buffsiz,
            content:    Vec::with_capacity(area)
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let (cols, rows) = size()?;

    execute!(stdout(), Clear(ClearType::All))?;

    for x in 0..cols {
        for y in 0..rows {
            execute!(stdout(), cursor::MoveTo(x, y))?;
            print!("x");
        }
    }


    Ok(()) 
}
