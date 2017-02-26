
pub trait Simulation {
    fn state(&self) -> [[u8; 10]; 10];
    fn advance(&mut self, generations: u8);
}

type Board = [[u8; 10]; 10];

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
            board_1: start,
            board_2: [[0u8; 10]; 10],
        }
    }

    fn next(&mut self) {
        let (from, mut to) = match self.current_field {
            FrontField::FieldOne => (&self.board_1, &mut self.board_2),
            FrontField::FieldTwo => (&self.board_2, &mut self.board_1),
        };

        for row in 0..10 {
            for col in 0..10 {
                let mut alive = 0;
                // test up
                if row > 0 {
                    // test up
                    if from[row - 1][col] == 1 {
                        alive += 1;
                    }
                    // test up left
                    if col > 0 {
                        if from[row - 1][col - 1] == 1 {
                            alive += 1;
                        }
                    }
                    // test up right
                    if col < 10 - 1 {
                        if from[row - 1][col + 1] == 1 {
                            alive += 1;
                        }
                    }
                }
                // test down
                if row < 10 - 1 {
                    // test down
                    if from[row + 1][col] == 1 {
                        alive += 1;
                    }
                    // test down left
                    if col > 0 {
                        if from[row + 1][col - 1] == 1 {
                            alive += 1;
                        }
                    }
                    // test down right
                    if col < 10 - 1 {
                        if from[row + 1][col + 1] == 1 {
                            alive += 1;
                        }
                    }
                }

                // test left
                if col > 0 {
                    if from[row][col - 1] == 1 {
                        alive += 1;
                    }
                }
                // test right
                if col < 10 - 1 {
                    if from[row][col + 1] == 1 {
                        alive += 1;
                    }
                }

                if alive == 0 {
                    to[row][col] = 0;
                } else if alive == 2 {
                    to[row][col] = from[row][col];
                } else if alive == 3 {
                    to[row][col] = 1;
                } else {
                    to[row][col] = 0;
                }
            }
        }

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

impl Simulation for Conway {
    fn state(&self) -> [[u8; 10]; 10] {
        match self.current_field {
            FrontField::FieldOne => self.board_1.clone(),
            FrontField::FieldTwo => self.board_2.clone(),
        }
    }

    fn advance(&mut self, generations: u8) {
        for _ in 0..generations {
            self.next();
        }
    }
}
