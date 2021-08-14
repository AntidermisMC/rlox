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
    no_equal: TokenType,
    equal: TokenType,
) -> TokenType {
    if next_is_equal(source) {
        equal
    } else {
        no_equal
    }
}

fn extend_with_digits(source: &mut LocationTrackingIterator<Peekable<Chars>>, s: &mut std::string::String) {
    let mut peek = source.peek();
    while peek.is_some() && peek.unwrap().is_ascii_digit() {
        s.push(source.next().unwrap());
        peek = source.peek();
    }
}

pub fn scan(source: &mut LocationTrackingIterator<Peekable<Chars>>, start: &mut Location) -> Result<Token, Error> {
    while let Some(char) = source.next() {
        return match char {

            // Comments
            '/' if source.peek() == Some(&'/') => {
                loop {
                    let next = source.next();
                    if next == Some('\n') {
                        *start = source.get_location();
                        break;
                    }
                    else if next == None {
                        return Ok(Token::new(EOF, consume_span(&mut source.get_location(), source.get_location())));
                    }
                }
                continue;
            }

            // Simple operators
            '(' => Ok(Token::new(LeftParen, consume_span(start, source.get_location()))),
            ')' => Ok(Token::new(RightParen, consume_span(start, source.get_location()))),
            '{' => Ok(Token::new(LeftBrace, consume_span(start, source.get_location()))),
            '}' => Ok(Token::new(RightBrace, consume_span(start, source.get_location()))),
            ',' => Ok(Token::new(Comma, consume_span(start, source.get_location()))),
            '.' => Ok(Token::new(Dot, consume_span(start, source.get_location()))),
            '-' => Ok(Token::new(Minus, consume_span(start, source.get_location()))),
            '+' => Ok(Token::new(Plus, consume_span(start, source.get_location()))),
            ';' => Ok(Token::new(Semicolon, consume_span(start, source.get_location()))),
            '*' => Ok(Token::new(Star, consume_span(start, source.get_location()))),
            '/' => Ok(Token::new(Slash, consume_span(start, source.get_location()))),

            // Composite operators
            '!' => Ok(Token::new(
                delimit_operator(source, Bang, BangEqual),
                consume_span(start, source.get_location()),
            )),
            '=' => Ok(Token::new(
                delimit_operator(source, Equal, EqualEqual),
                consume_span(start, source.get_location()),
            )),

            // Whitespace
            '\t' | ' ' => {
                *start = source.get_location();
                continue;
            }
            '\n' => {
                *start = source.get_location();
                continue;
            }

            // String literals
            '"' => {
                let mut str = std::string::String::new();
                while source.peek() != Some(&'"') {
                    if let Some(c) = source.next() {
                        str.push(c);
                    }
                    else {
                        let span = consume_span(start, source.get_location());
                        return Ok(Token::new(Invalid(Error::new("Unterminated string".to_string(), span)), span))
                    }
                }
                source.next();
                Ok(Token::new(TokenType::String(str), consume_span(start, source.get_location())))
            }

            // Number literals
            c if c.is_ascii_digit() => {
                let mut str = std::string::String::new();
                str.push(c);
                extend_with_digits(source, &mut str);
                if source.peek() == Some(&'.') {
                    str.push(source.next().unwrap());
                    extend_with_digits(source, &mut str);
                }
                Ok(Token::new(Number(str.parse::<f64>().unwrap()), consume_span(start, source.get_location())))
            }

            // Identifiers
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut str = std::string::String::new();
                str.push(c);
                loop {
                    match source.peek() {
                        Some(c) if c.is_ascii_alphanumeric() || *c == '_' => str.push(source.next().unwrap()),
                        _ => break,
                    }
                }
                Ok(Token::new(
                    match str.as_str() {
                        "and" => And,
                        "class" => Class,
                        "else" => Else,
                        "false" => False,
                        "for" => For,
                        "fun" => Fun,
                        "if" => If,
                        "nil" => Nil,
                        "or" => Or,
                        "print" => Print,
                        "return" => Return,
                        "super" => Super,
                        "this" => This,
                        "true" => True,
                        "var" => Var,
                        "while" => While,
                        _ => Identifier(str)
                    },
                    consume_span(start, source.get_location())
                ))
            }

            // Errors
            c => Ok(Token::new(
                Invalid(Error::new(
                    format!("Invalid character '{}'", c),
                    CodeSpan::new(Location::new(start.line, start.char), source.get_location()),
                )),
                CodeSpan::new(Location::new(start.line, start.char), source.get_location()),
            )),
        };
    }
    return Ok(Token::new(TokenType::EOF, consume_span(start, source.get_location())));
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
        let expected = "Token { token: EOF, span: ([1,0]) }\n";
        assert_equals(code, expected);
    }

    #[test]
    fn one_operator() {
        let code = "+";
        let expected = "\
        Token { token: Plus, span: ([1,0]-[1,1]) }\n\
        Token { token: EOF, span: ([1,1]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn two_char_operator() {
        let code = "!=";
        let expected = "\
        Token { token: BangEqual, span: ([1,0]-[1,2]) }\n\
        Token { token: EOF, span: ([1,2]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn bang_equal_equal() {
        let code = "!==";
        let expected = "Token { token: BangEqual, span: ([1,0]-[1,2]) }\n\
        Token { token: Equal, span: ([1,2]-[1,3]) }\n\
        Token { token: EOF, span: ([1,3]) }\n";
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
        Token { token: EOF, span: ([1,17]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_whitespace() {
        let code = "\t \n";
        let expected = "Token { token: EOF, span: ([2,0]) }\n";
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
        Token { token: EOF, span: ([2,3]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_comment() {
        let code = "// This is but a comment";
        let expected = "\
        Token { token: EOF, span: ([1,24]) }\n\
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
        Token { token: EOF, span: ([2,1]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_string() {
        let code = r#""a string""#;
        let expected = "\
        Token { token: String(\"a string\"), span: ([1,0]-[1,10]) }\n\
        Token { token: EOF, span: ([1,10]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn empty_string() {
        let code = r#""""#;
        let expected = "\
        Token { token: String(\"\"), span: ([1,0]-[1,2]) }\n\
        Token { token: EOF, span: ([1,2]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn string_and_operator() {
        let code = r#"""+"#;
        let expected = "\
        Token { token: String(\"\"), span: ([1,0]-[1,2]) }\n\
        Token { token: Plus, span: ([1,2]-[1,3]) }\n\
        Token { token: EOF, span: ([1,3]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn unterminated_string() {
        let code = r#""i swear i am compl"#;
        let expected = "\
        Token { token: Invalid(([1,0]-[1,19]) error: Unterminated string), span: ([1,0]-[1,19]) }\n\
        Token { token: EOF, span: ([1,19]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn integer() {
        let code = "0";
        let expected = "\
        Token { token: Number(0.0), span: ([1,0]-[1,1]) }\n\
        Token { token: EOF, span: ([1,1]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn float() {
        let code = "1.0";
        let expected = "\
        Token { token: Number(1.0), span: ([1,0]-[1,3]) }\n\
        Token { token: EOF, span: ([1,3]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn invalid_floats(){
        let code = ".1 1.";
        let expected = "\
        Token { token: Dot, span: ([1,0]-[1,1]) }\n\
        Token { token: Number(1.0), span: ([1,1]-[1,2]) }\n\
        Token { token: Number(1.0), span: ([1,3]-[1,5]) }\n\
        Token { token: EOF, span: ([1,5]) }\n\
        ";
        // Note: yes, it does accept integers with a trailing dot.
        // This could be fixed but would be ugly so I'm leaving it be for now.
        // TODO
        assert_equals(code, expected);
    }

    #[test]
    fn identifier() {
        let code = "Bond";
        let expected = "\
        Token { token: Identifier(\"Bond\"), span: ([1,0]-[1,4]) }\n\
        Token { token: EOF, span: ([1,4]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn many_identifiers() {
        let code = "\
        Bond James\n\
        b0nd _007\n\
        b _\
        ";
        let expected = "\
        Token { token: Identifier(\"Bond\"), span: ([1,0]-[1,4]) }\n\
        Token { token: Identifier(\"James\"), span: ([1,5]-[1,10]) }\n\
        Token { token: Identifier(\"b0nd\"), span: ([2,0]-[2,4]) }\n\
        Token { token: Identifier(\"_007\"), span: ([2,5]-[2,9]) }\n\
        Token { token: Identifier(\"b\"), span: ([3,0]-[3,1]) }\n\
        Token { token: Identifier(\"_\"), span: ([3,2]-[3,3]) }\n\
        Token { token: EOF, span: ([3,3]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn reserved_word() {
        let code = "if";
        let expected = "\
        Token { token: If, span: ([1,0]-[1,2]) }\n\
        Token { token: EOF, span: ([1,2]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn maximal_munch() {
        let code = "ifor";
        let expected = "\
        Token { token: Identifier(\"ifor\"), span: ([1,0]-[1,4]) }\n\
        Token { token: EOF, span: ([1,4]) }\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn all_reserved_word() {
        let code = "\
        and\n\
        class\n\
        else\n\
        false\n\
        for\n\
        fun\n\
        if\n\
        nil\n\
        or\n\
        print\n\
        return\n\
        super\n\
        this\n\
        true\n\
        var\n\
        while\n\
        ";
        let expected = "Token { token: And, span: ([1,0]-[1,3]) }\n\
        Token { token: Class, span: ([2,0]-[2,5]) }\n\
        Token { token: Else, span: ([3,0]-[3,4]) }\n\
        Token { token: False, span: ([4,0]-[4,5]) }\n\
        Token { token: For, span: ([5,0]-[5,3]) }\n\
        Token { token: Fun, span: ([6,0]-[6,3]) }\n\
        Token { token: If, span: ([7,0]-[7,2]) }\n\
        Token { token: Nil, span: ([8,0]-[8,3]) }\n\
        Token { token: Or, span: ([9,0]-[9,2]) }\n\
        Token { token: Print, span: ([10,0]-[10,5]) }\n\
        Token { token: Return, span: ([11,0]-[11,6]) }\n\
        Token { token: Super, span: ([12,0]-[12,5]) }\n\
        Token { token: This, span: ([13,0]-[13,4]) }\n\
        Token { token: True, span: ([14,0]-[14,4]) }\n\
        Token { token: Var, span: ([15,0]-[15,3]) }\n\
        Token { token: While, span: ([16,0]-[16,5]) }\n\
        Token { token: EOF, span: ([17,0]) }\n\
        ";
        assert_equals(code, expected);
    }
}
