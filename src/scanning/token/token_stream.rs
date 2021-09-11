use crate::location::Location;
use crate::location_tracking_iterator::LocationTrackingIterator;
use std::str::Chars;
use crate::scanning::{scan, Token};
use crate::scanning::token::token_stream::Position::{Index, End};

pub enum Position {
    End,
    Index(usize),
}

/// TokenStream is an iterator that returns lazily-scanned tokens and allows backtracking.
pub struct TokenStream<'a> {
    it: LocationTrackingIterator<Chars<'a>>,
    loc: Location,
    vec: Vec<Token>,
    pos: Position,
}

impl<'a> TokenStream<'a> {
    pub fn new(text: &'a str) -> Self {
        TokenStream {
            it: LocationTrackingIterator::new(text.chars()),
            loc: Location::start(),
            vec: vec![],
            pos: Position::End,
        }
    }

    pub fn back(&mut self) {
        if let Position::Index(n) = self.pos {
            assert_ne!(n, 0);
            self.pos = Index(n - 1);
        }
        else {
            assert_ne!(self.vec.len(), 0);
            self.pos = Index(self.vec.len() - 1);
        }
    }

    fn next_token(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(token) = scan(&mut self.it, &mut self.loc) {
            self.vec.push(token);
            Some(self.vec.last().unwrap().clone()) // Last should NEVER return None
        } else {
            None
        }
    }

    pub fn set_pos(&mut self, pos: Position) {
        if let Index(n) = pos {
            assert!(n < self.vec.len());
        }
        self.pos = pos;
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = super::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Position::Index(n) = self.pos {
            let val = self.vec[n].clone();

            self.pos = if self.vec.len() == n + 1 {
                End
            }
            else {
                Index(n + 1)
            };
            Some(val)
        }
        else {
            self.next_token()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanning::{TokenStream, Token, scan_all};
    use crate::scanning::token::token_stream::Position::Index;

    #[test]
    fn next() {
        let text = "a = b + c";
        let token_stream = TokenStream::new(text);
        let vec = scan_all(text);
        assert_eq!(
            token_stream.collect::<Vec<Token>>(),
            vec.unwrap()
        )
    }

    #[test]
    fn single_back() {
        let text = "a = b + c";
        let expected = "\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,2]-[1,3] Equal\n\
        [1,2]-[1,3] Equal\n\
        [1,4]-[1,5] Identifier(\"b\")\n\
        [1,6]-[1,7] Plus\n\
        [1,8]-[1,9] Identifier(\"c\")\n\
        ";
        let mut token_stream = TokenStream::new(text);
        let mut vec = vec![];
        vec.push(token_stream.next().unwrap());
        vec.push(token_stream.next().unwrap());
        token_stream.back();
        vec.extend(token_stream);
        assert_eq!(crate::scanning::to_string(vec), expected);
    }

    #[test]
    fn set_pos_then_back() {
        let text = "a = b + c";
        let expected = "\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,2]-[1,3] Equal\n\
        [1,4]-[1,5] Identifier(\"b\")\n\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,2]-[1,3] Equal\n\
        [1,4]-[1,5] Identifier(\"b\")\n\
        [1,6]-[1,7] Plus\n\
        [1,8]-[1,9] Identifier(\"c\")\n\
        ";
        let mut token_stream = TokenStream::new(text);
        let mut vec = vec![];
        vec.push(token_stream.next().unwrap());
        vec.push(token_stream.next().unwrap());
        vec.push(token_stream.next().unwrap());
        token_stream.set_pos(Index(1));
        token_stream.back();
        vec.extend(token_stream);
        assert_eq!(crate::scanning::to_string(vec), expected);
    }

    #[test]
    #[should_panic]
    fn back_from_start() {
        TokenStream::new("a").back()
    }
}
