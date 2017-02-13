

// [0, 1] == row 0 column 1

#[cfg(test)]
mod tests {
    #[test]
    fn glider_test() {
        let mut gen_0 = [[0u8; 10]; 10];
        // __X
        // X_X
        // _XX
        gen_0[0][2] = 1;
        gen_0[1][0] = 1;
        gen_0[1][2] = 1;
        gen_0[2][1] = 1;
        gen_0[2][2] = 1;
        let mut gen_1 = [[0u8; 10]; 10];
        // _X__
        // __XX
        // _XX_
        gen_1[0][1] = 1;
        gen_1[1][2] = 1;
        gen_1[1][3] = 1;
        gen_1[2][1] = 1;
        gen_1[2][2] = 1;

        let res_1 = super::get_next_gen(&gen_0);
        assert_eq!(gen_1, res_1);
    }
}


fn get_next_gen(field: &[[u8; 10]; 10]) -> [[u8; 10]; 10] {
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
