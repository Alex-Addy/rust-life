
pub trait Simulation {
    fn state(&self) -> Board;
    fn advance(&mut self, generations: u8);
}

type Board = Vec<Vec<u8>>;

pub mod simple;
