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

    pub fn insert(&mut self, chr: char) {
        self.buffer[self.gap_start] = chr;
        self.gap_start += 1;
        self.length += 1;
    }

    pub fn insert_str(&mut self, string: &str) {
        self.length += string.len();

        for chr in string.chars() {
            self.buffer[self.gap_start] = chr;
            self.gap_start += 1;
        }
    }

    pub fn move_to(&mut self, new_idx: usize) {
        if new_idx < self.length {
            // Move all values past the new index to the gap end
            if new_idx < self.gap_start {
                for i in new_idx..self.gap_start {
                    let after_gap_idx = i + (self.gap_end - self.gap_start) + 1;

                    self.buffer[after_gap_idx] = self.buffer[i];
                    self.buffer[i] = char::default();
                }

                self.gap_end -= self.gap_start - new_idx;
            }
            // Move all values past the gap end to their original position (up to the new index)
            else if new_idx > self.gap_start {
                for i in (self.gap_end + 1)..(BUFFER_LEN - (self.length - new_idx)) {
                    let before_gap_idx = i - (self.gap_end - (new_idx - self.gap_start)) - 1;

                    self.buffer[before_gap_idx] = self.buffer[i];
                    self.buffer[i] = char::default();
                }

                self.gap_end += self.length - new_idx - 1;
            }

            // self.current_idx = new_idx;
            self.gap_start = new_idx;
        }
        // New index is within the gap, but past the length
        // --> Force all chars to be connected (Cant write to arbitrary parts of the gap)
        else if new_idx > self.length {
            for i in (self.gap_end + 1)..BUFFER_LEN {
                let before_gap_idx = i - (self.gap_end - self.gap_start) - 1;

                self.buffer[before_gap_idx] = self.buffer[i];
                self.buffer[i] = char::default();
            }

            // self.current_idx = self.length;
            self.gap_start = self.length;
            self.gap_end = BUFFER_LEN - 1;
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
