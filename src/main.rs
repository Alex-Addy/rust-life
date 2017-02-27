use std::io;

extern crate gameoflife;
use gameoflife::simulate;
use gameoflife::simulate::Simulation;
use gameoflife::display;

fn main() {
    let start = [[0u8; 10]; 10];
    let mut field = simulate::Conway::new(start);
    field.advance(1);

    let mut stdout = io::stdout();
    display::dump_board(&mut stdout, field.state()).unwrap_or_else(|e: io::Error| {
        println!("{}", e);
    });
}
