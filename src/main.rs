pub const BUFFER_LEN: usize = 1024;

pub struct GapBuffer {
    pub buffer: [char; BUFFER_LEN],
    pub current_idx: usize,
    pub length: usize,
    pub gap_start: usize,
    pub gap_end: usize,
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self {
            buffer: [char::default(); BUFFER_LEN],
            current_idx: 0,
            length: 0,
            gap_start: 0,
            gap_end: BUFFER_LEN - 1,
        }
    }
}

impl GapBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, chr: char) {
        // If the inserted char is not within the gap
        if self.current_idx <= self.gap_start {
            self.buffer[self.current_idx] = chr;
            self.current_idx += 1;
            self.gap_start += 1;
            self.length += 1;
        } else {
            todo!()
        }
    }

    pub fn insert_str(&mut self, string: &str) {
        self.gap_start += string.len();
        self.length += string.len();

        for chr in string.chars() {
            if self.current_idx <= self.gap_start {
                self.buffer[self.current_idx] = chr;
                self.current_idx += 1;
            } else {
                todo!()
            }
        }
    }

    pub fn move_to(&mut self, new_idx: usize) {
        // Move all values past the new index to the gap end
        if new_idx < self.length {
            if new_idx < self.gap_start {
                self.gap_end = BUFFER_LEN - 1 - (self.length - new_idx);

                for i in new_idx..=(self.length - new_idx) {
                    let after_gap_idx = i + self.gap_end;

                    self.buffer[after_gap_idx] = self.buffer[i];
                    self.buffer[i] = char::default();
                }
            } else if new_idx > self.gap_start {
                for i in (self.gap_end + 1)..(BUFFER_LEN - (self.length - new_idx)) {
                    let before_gap_idx = i - self.gap_end;

                    self.buffer[before_gap_idx] = self.buffer[i];
                    self.buffer[i] = char::default();
                }
            }

            self.current_idx = new_idx;
            self.gap_start = new_idx;
        } else {
            // Force the chars to be connected to the other chars (Cant write to arbitrary parts of the gap)

            for i in (self.gap_end + 1)..BUFFER_LEN {
                let before_gap_idx = i - self.gap_end + (self.length - self.gap_start);

                self.buffer[before_gap_idx] = self.buffer[i];
                self.buffer[i] = char::default();
            }

            self.current_idx = self.length;
            self.gap_start = self.length;
        }
    }
}

impl std::fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = String::new();

        for i in 0..self.gap_start {
            message.push(self.buffer[i]);
        }

        for i in self.gap_end..BUFFER_LEN {
            message.push(self.buffer[i]);
        }

        write!(f, "{message}")
    }
}

impl std::fmt::Debug for GapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = String::new();

        for i in 0..self.gap_start {
            message.push(self.buffer[i]);
        }

        // message += ".".repeat(self.gap_end - self.gap_start).as_str();
        message += ".".repeat(10).as_str();

        for i in self.gap_end..BUFFER_LEN {
            message.push(self.buffer[i]);
        }

        write!(f, "{message}")
    }
}

fn main() {
    let mut buf = GapBuffer::new();

    buf.insert_str("ABC");

    println!("{buf}");

    buf.move_to(1);
    buf.insert('D');
    println!("{buf}");

    buf.move_to(0);
    buf.insert('E');
    println!("{buf}");

    buf.move_to(10);
    buf.insert('F');

    println!("{buf}");
}
