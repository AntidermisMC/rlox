pub mod token;

use crate::code_span::CodeSpan;
use crate::error::Error;
use crate::location::Location;
use crate::scanning::token::TokenType;
use crate::scanning::token::TokenType::*;
use token::Token;
use crate::location_tracking_iterator::LocationTrackingIterator;
use std::str::Chars;
use std::iter::Peekable;

/// Returns the current span and starts a new one.
fn consume_span(start: &mut Location, end: Location) -> CodeSpan {
    let span = CodeSpan::new(*start, end);
    *start = end;
    span
}

fn next_is_equal(it: &mut LocationTrackingIterator<Peekable<Chars>>) -> bool {
    match it.peek() {
        Some(c) if *c == '=' => {
            it.next();
            true
        }
        _ => false,
    }
}

fn delimit_operator(
    source: &mut LocationTrackingIterator<Peekable<Chars>>,
    current: &mut Location,
    no_equal: TokenType,
    equal: TokenType,
) -> TokenType {
    if next_is_equal(source) {
        current.char += 1;
        equal
    } else {
        no_equal
    }
}

pub fn scan(source: &mut LocationTrackingIterator<Peekable<Chars>>, start: &mut Location) -> Result<Token, Error> {
    let mut current = *start;
    while let Some(char) = source.next() {
        current.char += 1;
        return match char {

            // Comments
            '/' if source.peek() == Some(&'/') => {
                loop {
                    let next = source.next();
                    if next == Some('\n') {
                        current.new_line();
                        *start = current;
                        break;
                    }
                    else if next == None {
                        start.char = current.char - 1;
                        return Ok(Token::new(EOF, consume_span(start, current)));
                    }
                    else {
                        current.char += 1;
                    }
                }
                continue;
            }

            // Simple operators
            '(' => Ok(Token::new(LeftParen, consume_span(start, current))),
            ')' => Ok(Token::new(RightParen, consume_span(start, current))),
            '{' => Ok(Token::new(LeftBrace, consume_span(start, current))),
            '}' => Ok(Token::new(RightBrace, consume_span(start, current))),
            ',' => Ok(Token::new(Comma, consume_span(start, current))),
            '.' => Ok(Token::new(Dot, consume_span(start, current))),
            '-' => Ok(Token::new(Minus, consume_span(start, current))),
            '+' => Ok(Token::new(Plus, consume_span(start, current))),
            ';' => Ok(Token::new(Semicolon, consume_span(start, current))),
            '*' => Ok(Token::new(Star, consume_span(start, current))),
            '/' => Ok(Token::new(Slash, consume_span(start, current))),

            // Composite operators
            '!' => Ok(Token::new(
                delimit_operator(source, &mut current, Bang, BangEqual),
                consume_span(start, current),
            )),
            '=' => Ok(Token::new(
                delimit_operator(source, &mut current, Equal, EqualEqual),
                consume_span(start, current),
            )),

            // Whitespace
            '\t' | ' ' => {
                *start = current;
                continue;
            }
            '\n' => {
                current.new_line();
                *start = current;
                continue;
            }

            // Errors
            c => Ok(Token::new(
                Invalid(Error::new(
                    format!("Invalid character '{}'", c),
                    CodeSpan::new(Location::new(start.line, start.char), current),
                )),
                CodeSpan::new(Location::new(start.line, start.char), current),
            )),
        };
    }
    current.char += 1;
    return Ok(Token::new(TokenType::EOF, consume_span(start, current)));
}

/// Scans every token in the given source and returns either the first error or a vector of all scanner tokens.
pub fn scan_all(code: &str) -> Result<Vec<Token>, Error> {
    let mut source = LocationTrackingIterator::new(code.chars().peekable());
    let mut vec = Vec::new();
    let mut loc = Location::start();
    loop {
        match scan(&mut source, &mut loc) {
            Ok(token) => {
                let is_eof = token.is_of_type(TokenType::EOF);
                vec.push(token);
                if is_eof {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use crate::scanning::scan_all;
    use std::string::String;

    fn assert_equals(to_be_parsed: &str, expected: &str) {
        let parsed = scan_all(to_be_parsed).unwrap();
        let mut s = String::new();
        for token in parsed {
            s.extend(format!("{:?}", token).chars());
            s.push('\n');
        }
        assert_eq!(s, expected);
    }

    #[test]
    fn empty() {
        let code = "";
        let expected = "Token { token: EOF, span: ([1,0]-[1,1]) }\n";
        assert_equals(code, expected);
    }

    #[test]
    fn one_operator() {
        let code = "+";
        let expected = "\
        Token { token: Plus, span: ([1,0]-[1,1]) }\n\
        Token { token: EOF, span: ([1,1]-[1,2]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn two_char_operator() {
        let code = "!=";
        let expected = "\
        Token { token: BangEqual, span: ([1,0]-[1,2]) }\n\
        Token { token: EOF, span: ([1,2]-[1,3]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn bang_equal_equal() {
        let code = "!==";
        let expected = "Token { token: BangEqual, span: ([1,0]-[1,2]) }\n\
        Token { token: Equal, span: ([1,2]-[1,3]) }\n\
        Token { token: EOF, span: ([1,3]-[1,4]) }\n";
        assert_equals(code, expected);
    }

    #[test]
    fn all_operators() {
        let code = "(){},.-+;*!!====/";
        let expected = "\
        Token { token: LeftParen, span: ([1,0]-[1,1]) }\n\
        Token { token: RightParen, span: ([1,1]-[1,2]) }\n\
        Token { token: LeftBrace, span: ([1,2]-[1,3]) }\n\
        Token { token: RightBrace, span: ([1,3]-[1,4]) }\n\
        Token { token: Comma, span: ([1,4]-[1,5]) }\n\
        Token { token: Dot, span: ([1,5]-[1,6]) }\n\
        Token { token: Minus, span: ([1,6]-[1,7]) }\n\
        Token { token: Plus, span: ([1,7]-[1,8]) }\n\
        Token { token: Semicolon, span: ([1,8]-[1,9]) }\n\
        Token { token: Star, span: ([1,9]-[1,10]) }\n\
        Token { token: Bang, span: ([1,10]-[1,11]) }\n\
        Token { token: BangEqual, span: ([1,11]-[1,13]) }\n\
        Token { token: EqualEqual, span: ([1,13]-[1,15]) }\n\
        Token { token: Equal, span: ([1,15]-[1,16]) }\n\
        Token { token: Slash, span: ([1,16]-[1,17]) }\n\
        Token { token: EOF, span: ([1,17]-[1,18]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_whitespace() {
        let code = "\t \n";
        let expected = "Token { token: EOF, span: ([2,0]-[2,1]) }\n";
        assert_equals(code, expected);
    }

    #[test]
    fn whitespace_between_ops() {
        let code = "! =\n=\t=";
        let expected = "\
        Token { token: Bang, span: ([1,0]-[1,1]) }\n\
        Token { token: Equal, span: ([1,2]-[1,3]) }\n\
        Token { token: Equal, span: ([2,0]-[2,1]) }\n\
        Token { token: Equal, span: ([2,2]-[2,3]) }\n\
        Token { token: EOF, span: ([2,3]-[2,4]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_comment() {
        let code = "// This is but a comment";
        let expected = "\
        Token { token: EOF, span: ([1,23]-[1,24]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn comment_in_between_code() {
        let code = "\
        (// Another comment\n\
        )\
        ";
        let expected = "\
        Token { token: LeftParen, span: ([1,0]-[1,1]) }\n\
        Token { token: RightParen, span: ([2,0]-[2,1]) }\n\
        Token { token: EOF, span: ([2,1]-[2,2]) }\n\
        ";
        assert_equals(code, expected);
    }
}
