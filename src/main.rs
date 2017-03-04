use std::io;
use std::thread;
use std::time;

extern crate termion;
use termion::clear;
use termion::cursor;

extern crate rand;
use rand::Rng;

extern crate gameoflife;
use gameoflife::simulate;
use gameoflife::simulate::Simulation;
use gameoflife::display;

fn main() {
    let (horiz, vert) = termion::terminal_size().unwrap();
    let mut gen_0 = randomboard((vert - 1) as usize, horiz as usize);
    let mut field = simulate::Conway::new(gen_0);
    let mut stdout = io::stdout();

    for _ in 0..40 {
        print!("{}{}", cursor::Goto(1, 1), clear::AfterCursor);
        field.advance(1);
        display::dump_board(&mut stdout, field.state()).unwrap_or_else(|e: io::Error| {
            println!("{}", e);
        });
        thread::sleep(time::Duration::from_millis(400));
    }
}

fn randomboard(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut board = vec![vec![0u8; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if rng.gen() {
                board[r][c] = 1;
            } else {
                board[r][c] = 0;
            }
        }
    }

    board
}
