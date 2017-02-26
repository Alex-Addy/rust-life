extern crate gameoflife;

use gameoflife::simulate;
use gameoflife::simulate::Simulation;

// [0, 1] == row 0 column 1

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

    let mut field = simulate::Conway::new(gen_0);
    assert_eq!(gen_0, field.state());
    field.advance(1);
    let res_1 = field.state();
    assert_eq!(gen_1, res_1);
}
