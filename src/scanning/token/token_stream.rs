use crate::location::Location;
use crate::location_tracking_iterator::LocationTrackingIterator;
use crate::scanning::token::token_stream::Position::{End, Index};
use crate::scanning::{scan, Token};
use std::str::Chars;

pub enum Position {
    End,
    Index(usize),
}

pub struct TokenStreamState {
    position: usize,
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

    /// Goes back one iteration
    pub fn back(&mut self) {
        if let Position::Index(n) = self.pos {
            assert_ne!(n, 0);
            self.pos = Index(n - 1);
        } else {
            assert_ne!(self.vec.len(), 0);
            self.pos = Index(self.vec.len() - 1);
        }
    }

    /// Internal. Immediately scan next token from source
    fn parse_next_token(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(token) = scan(&mut self.it, &mut self.loc) {
            let clone = token.clone();
            self.vec.push(token);
            Some(clone) // Last should NEVER return None
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

    pub fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        let item = self.next();
        if item.is_some() {
            self.back();
        }
        item
    }

    pub fn save_position(&self) -> TokenStreamState {
        let position = match self.pos {
            Position::Index(n) => n,
            Position::End => self.vec.len(),
        };
        TokenStreamState { position }
    }

    pub fn load_position(&mut self, save: TokenStreamState) {
        if save.position == self.vec.len() {
            self.pos = Position::End;
        } else {
            self.pos = Position::Index(save.position);
        }
    }

    pub fn current_position(&self) -> Location {
        match self.vec.last() {
            None => Location::start(),
            Some(token) => token.span.end,
        }
    }

    pub fn has_next(&mut self) -> bool {
        match self.peek() {
            Some(_) => true,
            None => false,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = super::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Position::Index(n) = self.pos {
            let val = self.vec[n].clone();

            self.pos = if self.vec.len() == n + 1 {
                End
            } else {
                Index(n + 1)
            };
            Some(val)
        } else {
            self.parse_next_token()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanning::token::token_stream::Position::Index;
    use crate::scanning::{scan_all, Token, TokenStream};

    #[test]
    fn next() {
        let text = "a = b + c";
        let token_stream = TokenStream::new(text);
        let vec = scan_all(text);
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec)
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
    fn peek() {
        let text = "a = b + c";
        let expected = "\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,0]-[1,1] Identifier(\"a\")\n\
        ";
        let mut token_stream = TokenStream::new(text);
        let mut vec = vec![];
        vec.push(token_stream.peek().unwrap());
        vec.push(token_stream.peek().unwrap());
        vec.push(token_stream.next().unwrap());
        assert_eq!(crate::scanning::to_string(vec), expected);

        for _ in 0..4 {
            token_stream.next();
        }
        assert_eq!(token_stream.peek(), None);
    }

    #[test]
    fn save_load() {
        let text = "a = b + c";
        let expected = "\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,2]-[1,3] Equal\n\
        [1,0]-[1,1] Identifier(\"a\")\n\
        [1,2]-[1,3] Equal\n\
        [1,4]-[1,5] Identifier(\"b\")\n\
        [1,6]-[1,7] Plus\n\
        [1,8]-[1,9] Identifier(\"c\")\n\
        ";
        let mut token_stream = TokenStream::new(text);
        let mut vec = vec![];

        let save = token_stream.save_position();
        vec.push(token_stream.next().unwrap());
        vec.push(token_stream.next().unwrap());
        token_stream.load_position(save);
        vec.extend(token_stream);

        assert_eq!(crate::scanning::to_string(vec), expected);
    }

    #[ignore] // Panic pause when running tests is annoying. TODO another day: find a better way.
    #[test]
    #[should_panic]
    fn back_from_start() {
        TokenStream::new("a").back()
    }

    #[test]
    fn eof_peek_then_next() {
        let text = "1";
        let mut token_stream = TokenStream::new(text);

        token_stream.next();
        token_stream.peek();
        assert_eq!(token_stream.next(), None);
    }
}
