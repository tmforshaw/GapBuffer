use crate::gap_buffer::GapBuffer;

pub mod gap_buffer;
fn main() {
    let mut buf = GapBuffer::new();

    buf.insert_str("ABC");
    println!("{buf:?}");

    buf.move_to(1);
    println!("{buf:?}");

    buf.insert_str("DE");
    println!("{buf:?}");

    buf.move_to(0);
    println!("{buf:?}");

    buf.insert('F');
    println!("{buf:?}");

    buf.move_to(10);
    println!("{buf:?}");

    buf.insert('G');
    println!("{buf:?}");

    buf.move_to(2);
    println!("{buf:?}");

    buf.move_to(4);
    println!("{buf:?}");

    buf.insert('H');
    println!("{buf:?}");

    buf.move_to(15);
    println!("{buf:?}");

    buf.insert('I');
    println!("{buf:?}");
}
