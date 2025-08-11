use crate::tui::init_tui;

pub mod gap_buffer;
pub mod tui;

fn main() {
    init_tui().unwrap();
}
