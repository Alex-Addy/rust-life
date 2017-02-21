

pub mod simulate {
    pub fn get_next_gen(field: &[[u8; 10]; 10]) -> [[u8; 10]; 10] {
        let mut next: [[u8; 10]; 10] = [[0u8; 10]; 10];

        for row in 0..10 {
            for col in 0..10 {
                let mut alive = 0;
                // test up
                if row > 0 {
                    // test up
                    if field[row - 1][col] == 1 {
                        alive += 1;
                    }
                    // test up left
                    if col > 0 {
                        if field[row - 1][col - 1] == 1 {
                            alive += 1;
                        }
                    }
                    // test up right
                    if col < 10 - 1 {
                        if field[row - 1][col + 1] == 1 {
                            alive += 1;
                        }
                    }
                }
                // test down
                if row < 10 - 1 {
                    // test down
                    if field[row + 1][col] == 1 {
                        alive += 1;
                    }
                    // test down left
                    if col > 0 {
                        if field[row + 1][col - 1] == 1 {
                            alive += 1;
                        }
                    }
                    // test down right
                    if col < 10 - 1 {
                        if field[row + 1][col + 1] == 1 {
                            alive += 1;
                        }
                    }
                }

                // test left
                if col > 0 {
                    if field[row][col - 1] == 1 {
                        alive += 1;
                    }
                }
                // test right
                if col < 10 - 1 {
                    if field[row][col + 1] == 1 {
                        alive += 1;
                    }
                }

                if alive == 0 {
                    next[row][col] = 0;
                } else if alive == 2 {
                    next[row][col] = field[row][col];
                } else if alive == 3 {
                    next[row][col] = 1;
                } else {
                    next[row][col] = 0;
                }
            }
        }

        next
    }

}
