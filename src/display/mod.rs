use std::io::Write;
use std::io;

pub fn dump_board<T: Write>(w: &mut T, board: Vec<Vec<u8>>) -> io::Result<()> {
    let dead = '□';
    let live = '■';
    let unknown = '?';

    let mut buf = String::new();
    for row in board.iter() {
        for cell in row.iter() {
            let c = match *cell {
                0 => dead,
                1 => live,
                _ => unknown,
            };
            buf.push(c);
        }
        buf.push('\n');
        w.write(&buf.as_bytes())?;
        buf.clear();
    }

    Ok(())
}
