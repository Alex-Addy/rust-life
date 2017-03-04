use std::io;
use std::thread;
use std::time;

extern crate termion;
use termion::clear;
use termion::cursor;

extern crate gameoflife;
use gameoflife::simulate;
use gameoflife::simulate::Simulation;
use gameoflife::display;

fn main() {
    let mut gen_0 = vec![vec![0u8; 40]; 30];
    // __X
    // X_X
    // _XX
    gen_0[0][2] = 1;
    gen_0[1][0] = 1;
    gen_0[1][2] = 1;
    gen_0[2][1] = 1;
    gen_0[2][2] = 1;
    let mut field = simulate::Conway::new(gen_0);
    let mut stdout = io::stdout();

    for _ in 0..40 {
        print!("{}{}", cursor::Goto(1, 1), clear::AfterCursor);
        field.advance(1);
        display::dump_board(&mut stdout, field.state()).unwrap_or_else(|e: io::Error| {
            println!("{}", e);
        });
        thread::sleep(time::Duration::from_millis(500));
    }

}
