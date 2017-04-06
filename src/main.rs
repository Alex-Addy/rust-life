use std::io;
use std::thread;
use std::time;

extern crate termion;
use termion::clear;
use termion::cursor;


extern crate clap;
use clap::{Arg, App};

extern crate gameoflife;
use gameoflife::simulate::Simulation;
use gameoflife::simulate::simple;
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

    let mut field = match matches.value_of("file") {
        Some(path) => {
            match simple::board::from_file(path) {
                Ok(b) => simple::Vec2d::new(b),
                Err(e) => {
                    println!("Could not load from file: {}", e);
                    return;
                }
            }
        }
        None => {
            let (horiz, vert) = termion::terminal_size().unwrap();
            let gen_0 = simple::board::randomboard((vert - 1) as usize, horiz as usize);
            simple::Vec2d::new(gen_0)
        }
    };
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
