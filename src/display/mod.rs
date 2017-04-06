use std::io;
use std::io::Write;

extern crate termion;
use self::termion::clear;
use self::termion::cursor;

pub trait Display {
    fn display_board(&mut self, board: Vec<Vec<u8>>) -> io::Result<()>;
}


pub struct Terminal<T: Write> {
    w: T,
}

impl<W: Write> Terminal<W> {
    pub fn new(writer: W) -> Terminal<W> {
        Terminal { w: writer }
    }
}

impl<W: Write> Display for Terminal<W> {
    fn display_board(&mut self, board: Vec<Vec<u8>>) -> io::Result<()> {
        // clear screen efficiently
        print!("{}{}", cursor::Goto(1, 1), clear::AfterCursor);

        let dead = ' ';
        let live = '■';
        let unknown = '?';

        let mut buf = String::new();
        for row in board.iter() {
            for cell in row.iter() {
                let c = match *cell {
                    0 => dead,
                    1 => live,
                    _ => unknown,
                };
                buf.push(c);
            }
            buf.push('\n');
            self.w.write(&buf.as_bytes())?;
            buf.clear();
        }

        Ok(())
    }
}
