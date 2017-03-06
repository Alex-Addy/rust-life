

pub trait Simulation {
    fn state(&self) -> Board;
    fn advance(&mut self, generations: u8);
}

type Board = Vec<Vec<u8>>;

pub mod board {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    extern crate rand;
    use self::rand::Rng;

    use simulate::Board;

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Board, String> {
        let mut board: Board = Vec::new();
        let mut row = Vec::new();
        let mut buf = String::new();
        try!(File::open(path)
            .and_then(|mut file| file.read_to_string(&mut buf))
            .map_err(|err| err.to_string()));
        for c in buf.chars() {
            match c {
                '_' | '0' => {
                    row.push(0);
                }
                'X' | '1' => {
                    row.push(1);
                }
                '\n' => {
                    board.push(row);
                    row = Vec::new();
                }
                _ => {}
            }
        }
        match validate_board(&board) {
            Ok(()) => Ok(board),
            Err(e) => Err(e.to_string()),
        }
    }

    fn validate_board(b: &Board) -> Result<(), String> {
        if b.len() == 0 {
            return Err("board is empty".to_owned());
        }

        let row_length = b[0].len();
        for i in 0..b.len() {
            let ref row = b[i];
            if row.len() == 0 {
                return Err(format!("row {} is empty", i));
            }
            if row.len() != row_length {
                return Err(format!("row {} does not have expected length, expected {} got {}",
                                   i,
                                   row_length,
                                   row.len()));
            }
        }

        return Ok(());
    }

    pub fn randomboard(rows: usize, cols: usize) -> Vec<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut board = vec![vec![0u8; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                if rng.gen() {
                    board[r][c] = 1;
                } else {
                    board[r][c] = 0;
                }
            }
        }

        board
    }
}

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
