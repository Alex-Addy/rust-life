
pub trait Simulation {
    fn state(&self) -> Vec<Vec<u8>>;
    fn advance(&mut self, generations: u8);
}

type Board = Vec<Vec<u8>>;

#[derive(Debug)]
enum FrontField {
    FieldOne,
    FieldTwo,
}


#[derive(Debug)]
pub struct Conway {
    current_field: FrontField,
    board_1: Board,
    board_2: Board,
}

impl Conway {
    pub fn new(start: Board) -> Conway {
        Conway {
            current_field: FrontField::FieldOne,
            board_2: vec![vec![0u8; start[0].len()]; start.len()],
            board_1: start,
        }
    }

    fn next(&mut self) {
        let (from, mut to) = match self.current_field {
            FrontField::FieldOne => (&self.board_1, &mut self.board_2),
            FrontField::FieldTwo => (&self.board_2, &mut self.board_1),
        };

        calc_gen(from, to);

        match self.current_field {
            FrontField::FieldOne => {
                self.current_field = FrontField::FieldTwo;
            }
            FrontField::FieldTwo => {
                self.current_field = FrontField::FieldOne;
            }
        };
    }
}

// calc_gen calculates the new generation using
// prev and places the new gen into next
// assumes prev and next have the same sizes
// assumes row and column number greater than 0
// assumes rectangle with all columns having the same height
fn calc_gen(prev: &Board, next: &mut Board) {
    let row_num = prev.len();
    let col_num = prev[0].len();
    for row in 0..row_num {
        for col in 0..col_num {
            let mut alive = 0;
            // test up
            if row > 0 {
                // test up
                if prev[row - 1][col] == 1 {
                    alive += 1;
                }
                // test up left
                if col > 0 {
                    if prev[row - 1][col - 1] == 1 {
                        alive += 1;
                    }
                }
                // test up right
                if col < col_num - 1 {
                    if prev[row - 1][col + 1] == 1 {
                        alive += 1;
                    }
                }
            }
            // test down
            if row < row_num - 1 {
                // test down
                if prev[row + 1][col] == 1 {
                    alive += 1;
                }
                // test down left
                if col > 0 {
                    if prev[row + 1][col - 1] == 1 {
                        alive += 1;
                    }
                }
                // test down right
                if col < col_num - 1 {
                    if prev[row + 1][col + 1] == 1 {
                        alive += 1;
                    }
                }
            }

            // test left
            if col > 0 {
                if prev[row][col - 1] == 1 {
                    alive += 1;
                }
            }
            // test right
            if col < col_num - 1 {
                if prev[row][col + 1] == 1 {
                    alive += 1;
                }
            }

            if alive == 0 {
                next[row][col] = 0;
            } else if alive == 2 {
                next[row][col] = prev[row][col];
            } else if alive == 3 {
                next[row][col] = 1;
            } else {
                next[row][col] = 0;
            }
        }
    }
}

impl Simulation for Conway {
    fn state(&self) -> Board {
        let b = match self.current_field {
            FrontField::FieldOne => &self.board_1,
            FrontField::FieldTwo => &self.board_2,
        };

        b.clone()
    }

    fn advance(&mut self, generations: u8) {
        for _ in 0..generations {
            self.next();
        }
    }
}
