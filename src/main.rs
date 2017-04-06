use std::io;

extern crate termion;

extern crate clap;
use clap::{Arg, App};

extern crate gameoflife;
use gameoflife::simulate::simple;
use gameoflife::display;
use gameoflife::engine;

fn main() {
    let matches = App::new("Conway's Game of Life")
        .author("Alex A. <alex.addy@gmail.com>")
        .arg(Arg::with_name("file")
            .short("f")
            .long("board-file")
            .help("file to load starting board from")
            .takes_value(true))
        .get_matches();

    let field = match matches.value_of("file") {
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

    let term = display::Terminal::new(io::stdout());
    let engine = engine::ExecutionEngine::new(field, term);
    engine.run();
}
