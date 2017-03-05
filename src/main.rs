use std::io;
use std::thread;
use std::time;
use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate termion;
use termion::clear;
use termion::cursor;

extern crate rand;
use rand::Rng;

extern crate clap;
use clap::{Arg, App};

extern crate gameoflife;
use gameoflife::simulate;
use gameoflife::simulate::Simulation;
use gameoflife::display;

fn main() {
    let matches = App::new("Conway's Game of Life")
        .author("Alex A. <alex.addy@gmail.com>")
        .arg(Arg::with_name("file")
            .short("f")
            .long("board-file")
            .help("file to load starting board from")
            .takes_value(true))
        .get_matches();

    let gen_0 = match matches.value_of("file") {
        Some(path) => {
            match load_board_from_file(path) {
                Ok(b) => b,
                Err(e) => {
                    println!("Could not load board: {}", e);
                    return;
                }
            }
        }
        None => {
            let (horiz, vert) = termion::terminal_size().unwrap();
            randomboard((vert - 1) as usize, horiz as usize)
        }
    };
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

fn load_board_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<u8>>, String> {

    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut row = Vec::new();
    let mut buf = String::new();
    try!(File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buf))
        .map_err(|err| err.to_string()));
    for c in buf.chars() {
        match c {
            '_' | '0' => {
                row.push(0);
            }
            'X' | '1' => {
                row.push(1);
            }
            '\n' => {
                board.push(row);
                row = Vec::new();
            }
            _ => {}
        }
    }

    match validate_board(&board) {
        Ok(()) => Ok(board),
        Err(e) => Err(e.to_string()),
    }
}

fn validate_board(b: &Vec<Vec<u8>>) -> Result<(), String> {
    if b.len() == 0 {
        return Err("board is empty".to_owned());
    }

    let row_length = b[0].len();
    for i in 0..b.len() {
        let ref row = b[i];
        if row.len() == 0 {
            return Err(format!("row {} is empty", i));
        }
        if row.len() != row_length {
            return Err(format!("row {} does not have expected length, expected {} got {}",
                               i,
                               row_length,
                               row.len()));
        }
    }

    return Ok(());
}
