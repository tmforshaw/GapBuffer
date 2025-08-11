pub const BUFFER_LEN: usize = 1024;

pub struct GapBuffer {
    pub buffer: [char; BUFFER_LEN],
    pub length: usize,
    pub gap_start: usize,
    pub gap_end: usize,
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self {
            buffer: [char::default(); BUFFER_LEN],
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

    pub fn gap_size(&self) -> usize {
        self.gap_end.saturating_sub(self.gap_start) + 1
    }

    pub fn insert(&mut self, chr: char) {
        if self.gap_size() == 0 {
            panic!("Could not insert char, gap size too small");
        }

        self.buffer[self.gap_start] = chr;
        self.gap_start += 1;
        self.length += 1;
    }

    pub fn insert_str<S: AsRef<str>>(&mut self, string: S) {
        let string = string.as_ref();

        if self.gap_size() < string.len() {
            panic!("Could not insert string, gap size too small: {}", self.gap_size());
        }

        self.length += string.len();

        for chr in string.chars() {
            self.buffer[self.gap_start] = chr;
            self.gap_start += 1;
        }
    }

    // Removes the char before the gap_start
    pub fn remove(&mut self) {
        if self.gap_start > 0 {
            self.length -= 1;
            self.gap_start -= 1;

            self.buffer[self.gap_start] = '\0';
        }
    }

    // Remove N chars before gap_start
    pub fn remove_n(&mut self, n: usize) {
        if n > 0 && self.gap_start > n - 1 {
            self.length -= n;

            for _ in (self.gap_start - n)..self.gap_start {
                self.gap_start -= 1;
                self.buffer[self.gap_start] = char::default();
            }
        }
    }

    pub fn move_to(&mut self, new_idx: usize) {
        // Don't need to do anything if the move doesn't actually move anywhere
        if new_idx == self.gap_start {
            return;
        }

        if new_idx > self.length {
            // Moved past the text length (Move gap to end of text)
            for i in (self.gap_end + 1)..BUFFER_LEN {
                let before_gap_idx = i - (self.gap_end - self.gap_start) - 1;

                self.buffer[before_gap_idx] = self.buffer[i];
                self.buffer[i] = char::default();
            }

            self.gap_start = self.length;
            self.gap_end = BUFFER_LEN - 1;
        } else {
            let shift = self.gap_start.abs_diff(new_idx);

            if new_idx < self.gap_start {
                // Moving gap left
                for i in 0..shift {
                    self.buffer[self.gap_end - i] = self.buffer[self.gap_start - i - 1];
                    self.buffer[self.gap_start - i - 1] = char::default();
                }

                self.gap_end -= shift;
            } else if new_idx > self.gap_start {
                // Moving gap right
                for i in 0..shift {
                    self.buffer[self.gap_start] = self.buffer[self.gap_end + i + 1];
                    self.buffer[self.gap_end + i + 1] = char::default();

                    self.gap_start += 1;
                }

                self.gap_end += shift;
            }

            self.gap_start = new_idx;
        }
    }
}

impl std::fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = String::new();

        for i in 0..self.gap_start {
            message.push(self.buffer[i]);
        }

        for i in (self.gap_end + 1)..BUFFER_LEN {
            message.push(self.buffer[i]);
        }

        write!(f, "{message}")
    }
}

impl std::fmt::Debug for GapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = String::new();

        for i in 0..(self.gap_start) {
            message.push(match self.buffer[i] {
                '\0' => '-',
                chr => chr,
            });
        }

        // message += ".".repeat(self.gap_end - self.gap_start).as_str();
        message += ".".repeat(3).as_str();

        for i in (self.gap_end + 1)..BUFFER_LEN {
            message.push(match self.buffer[i] {
                '\0' => '_',
                chr => chr,
            });
        }

        write!(f, "{message}")
    }
}

#[test]
fn test_gap_buffer() {
    let mut buf = GapBuffer::new();

    buf.insert_str("ABCD");
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

    buf.move_to(2);
    println!("{buf:?}");

    buf.remove();
    println!("{buf:?}");

    buf.move_to(2);
    println!("{buf:?}");

    buf.move_to(3);
    buf.move_to(3);
    println!("{buf:?}");
    buf.remove_n(3);
    println!("{buf:?}");
}
