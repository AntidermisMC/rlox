use std::iter::Peekable;
use std::str::Chars;
use crate::location::Location;

pub struct Source<'a> {
    text: Peekable<Chars<'a>>,
    location: Location,
}

impl<'a> Source<'a> {
    pub fn new(lines: &'a str) -> Self {
        Source {
            text: lines.chars().peekable(),
            location: Location::start(),
        }
    }

    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.text.peek()
    }
}

impl<'a> Iterator for Source<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.text.next();
        match c {
            Some('\n') => {
                self.location.line += 1;
                self.location.char = 0;
            }
            Some(_) => self.location.char += 1,
            None => (),
        }
        c
    }
}

#[test]
fn new_empty() {
    assert_eq!(Source::new("").collect::<Vec<char>>(), vec![])
}

#[test]
fn new_one_line() {
    assert_eq!(
        Source::new("one").collect::<Vec<char>>(),
        vec!['o', 'n', 'e']
    )
}

#[test]
fn new_one_line_eol() {
    assert_eq!(
        Source::new("one\n").collect::<Vec<char>>(),
        vec!['o', 'n', 'e', '\n']
    )
}

#[test]
fn new_multiple_lines() {
    assert_eq!(
        Source::new("one\ntwo").collect::<Vec<char>>(),
        "one\ntwo".chars().collect::<Vec<char>>()
    )
}
