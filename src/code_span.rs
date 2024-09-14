use std::fmt::{Debug, Display, Formatter};

use crate::location::Location;

/// Represents a span of code over the source, possibly over multiple lines.
#[derive(Copy, Clone, PartialEq)]
pub struct CodeSpan {
    pub start: Location,
    pub end: Location,
}

impl CodeSpan {
    pub fn new(start: Location, end: Location) -> Self {
        CodeSpan { start, end }
    }

    /// Returns whether the span is contained in one line.
    pub fn is_one_line(&self) -> bool {
        self.start.line == self.end.line
    }

    /// Returns the range of all lines in the span.
    pub fn lines(&self) -> std::ops::Range<usize> {
        self.start.line..self.end.line
    }

    /// Clones the representation of a token from its source.
    pub fn get_repr(&self, source: Vec<&str>) -> String {
        if self.is_one_line() {
            source[self.start.line][self.start.char..self.end.char].to_string()
        } else {
            let mut s = String::from(&source[self.start.line][self.start.char..]);
            let mut line = self.start.line + 1;
            while line < self.end.line {
                s.push_str(source[line]);
                line += 1;
            }
            s.push_str(&source[line][..self.end.char]);
            s
        }
    }

    pub fn combine(left: CodeSpan, right: CodeSpan) -> Self {
        CodeSpan {
            start: left.start,
            end: right.end,
        }
    }
}

impl Debug for CodeSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.start != self.end {
            write!(f, "{}-{}", self.start, self.end)
        } else {
            write!(f, "{}", self.start)
        }
    }
}

impl Display for CodeSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
