use std::thread;
use std::time;
use std::io;

use simulate::Simulation;
use display::Display;

#[derive(Debug)]
pub struct ExecutionEngine<S: Simulation, D: Display> {
    simulation: S,
    display: D,
}

impl<S: Simulation, D: Display> ExecutionEngine<S, D> {
    pub fn new(sim: S, disp: D) -> ExecutionEngine<S, D> {
        ExecutionEngine {
            simulation: sim,
            display: disp,
        }
    }

    pub fn run(mut self) -> usize {
        for _ in 0..40 {
            self.simulation.advance(1);
            self.display.display_board(self.simulation.state()).unwrap_or_else(|e: io::Error| {
                println!("{}", e);
            });
            thread::sleep(time::Duration::from_millis(400));
        }
        0
    }
}
