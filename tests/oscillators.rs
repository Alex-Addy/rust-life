
extern crate gameoflife;

use gameoflife::simulate;
use gameoflife::simulate::Simulation;

#[test]
fn blinker_test() {
    // _____
    // _XXX_
    // _____
    let mut gen_0 = vec![vec![0u8; 5]; 5];
    gen_0[1][1] = 1;
    gen_0[1][2] = 1;
    gen_0[1][3] = 1;
    // __X__
    // __X__
    // __X__
    let mut gen_1 = vec![vec![0u8; 5]; 5];
    gen_1[0][2] = 1;
    gen_1[1][2] = 1;
    gen_1[2][2] = 1;

    let mut field = simulate::Conway::new(gen_0.clone());
    assert_eq!(gen_0, field.state());
    field.advance(1);
    assert_eq!(gen_1, field.state());
    field.advance(1);
    assert_eq!(gen_0, field.state());

    field.advance(101);
    assert_eq!(gen_1, field.state());
}
