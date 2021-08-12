use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub char: usize,
}

impl Location {
    pub fn new(line: usize, char: usize) -> Self {
        Location { line, char }
    }

    pub fn start() -> Self {
        Location { line: 1, char: 0 }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.char = 0;
    }

    pub fn next_char(&mut self) {
        self.char += 1;
    }

    pub fn advance(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            _ => self.next_char(),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.line, self.char)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
