extern crate gameoflife;

use gameoflife::simulate;
use gameoflife::simulate::Simulation;

// [0, 1] == row 0 column 1

#[test]
fn glider_test() {
    // __X
    // X_X
    // _XX
    let mut gen_0 = vec![vec![0u8; 10]; 10];
    gen_0[0][2] = 1;
    gen_0[1][0] = 1;
    gen_0[1][2] = 1;
    gen_0[2][1] = 1;
    gen_0[2][2] = 1;
    // _X__
    // __XX
    // _XX_
    let mut gen_1 = vec![vec![0u8; 10]; 10];
    gen_1[0][1] = 1;
    gen_1[1][2] = 1;
    gen_1[1][3] = 1;
    gen_1[2][1] = 1;
    gen_1[2][2] = 1;
    // __X_
    // ___X
    // _XXX
    let mut gen_2 = vec![vec![0u8; 10]; 10];
    gen_2[0][2] = 1;
    gen_2[1][3] = 1;
    gen_2[2][1] = 1;
    gen_2[2][2] = 1;
    gen_2[2][3] = 1;

    let mut field = simulate::Conway::new(gen_0.clone());
    assert_eq!(gen_0, field.state());
    field.advance(1);
    assert_eq!(gen_1, field.state());
    field.advance(1);
    assert_eq!(gen_2, field.state());
}
