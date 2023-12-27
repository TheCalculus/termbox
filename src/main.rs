#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io::{self, Write, stdout};
use crossterm::{
    cursor,
    execute,
    terminal::{enable_raw_mode, size, Clear, ClearType}
};

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
