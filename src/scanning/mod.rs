mod scanning_error;
pub mod token;

use std::str::Chars;

pub use scanning_error::ScanningError;
pub use token::{token_stream::TokenStream, Token, TokenType};

use crate::{
    code_span::CodeSpan, location::Location, location_tracking_iterator::LocationTrackingIterator,
    scanning::token::TokenType::*,
};

/// Returns the current span and starts a new one.
fn consume_span(start: &mut Location, end: Location) -> CodeSpan {
    let span = CodeSpan::new(*start, end);
    *start = end;
    span
}

fn next_is_equal(it: &mut LocationTrackingIterator<Chars>) -> bool {
    match it.peek() {
        Some(c) if *c == '=' => {
            it.next();
            true
        }
        _ => false,
    }
}

/// Delimits an operator depending on whether the next character is '=' or not.
fn delimit_operator(
    source: &mut LocationTrackingIterator<Chars>,
    no_equal: TokenType,
    equal: TokenType,
) -> TokenType {
    if next_is_equal(source) {
        equal
    } else {
        no_equal
    }
}

fn extend_with_digits(source: &mut LocationTrackingIterator<Chars>, s: &mut std::string::String) {
    let mut peek = source.peek();
    while peek.is_some() && peek.unwrap().is_ascii_digit() {
        s.push(source.next().unwrap());
        peek = source.peek();
    }
}

/// Scans a text stream.
/// start should be Location::start() unless resuming from a previous iterator's
/// text.
pub fn scan(source: &mut LocationTrackingIterator<Chars>, start: &mut Location) -> Option<Token> {
    while let Some(char) = source.next() {
        return match char {
            // Comments
            '/' if source.peek() == Some(&'/') => {
                loop {
                    let next = source.next();
                    if next == Some('\n') {
                        *start = source.get_location();
                        break;
                    } else if next == None {
                        return None;
                    }
                }
                continue;
            }

            // Simple operators
            '(' => Some(Token::new(
                LeftParen,
                consume_span(start, source.get_location()),
            )),
            ')' => Some(Token::new(
                RightParen,
                consume_span(start, source.get_location()),
            )),
            '{' => Some(Token::new(
                LeftBrace,
                consume_span(start, source.get_location()),
            )),
            '}' => Some(Token::new(
                RightBrace,
                consume_span(start, source.get_location()),
            )),
            ',' => Some(Token::new(
                Comma,
                consume_span(start, source.get_location()),
            )),
            '.' => Some(Token::new(Dot, consume_span(start, source.get_location()))),
            '-' => Some(Token::new(
                Minus,
                consume_span(start, source.get_location()),
            )),
            '+' => Some(Token::new(Plus, consume_span(start, source.get_location()))),
            ';' => Some(Token::new(
                Semicolon,
                consume_span(start, source.get_location()),
            )),
            '*' => Some(Token::new(Star, consume_span(start, source.get_location()))),
            '/' => Some(Token::new(
                Slash,
                consume_span(start, source.get_location()),
            )),

            // Composite operators
            '!' => Some(Token::new(
                delimit_operator(source, Bang, BangEqual),
                consume_span(start, source.get_location()),
            )),
            '=' => Some(Token::new(
                delimit_operator(source, Equal, EqualEqual),
                consume_span(start, source.get_location()),
            )),
            '<' => Some(Token::new(
                delimit_operator(source, Less, LessEqual),
                consume_span(start, source.get_location()),
            )),
            '>' => Some(Token::new(
                delimit_operator(source, Greater, GreaterEqual),
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
                    } else {
                        let span = consume_span(start, source.get_location());
                        return Some(Token::new(
                            Invalid(ScanningError::UnterminatedString(span)),
                            span,
                        ));
                    }
                }
                source.next();
                Some(Token::new(
                    TokenType::String(str),
                    consume_span(start, source.get_location()),
                ))
            }

            // Number literals
            c if c.is_ascii_digit() => {
                let mut str = std::string::String::new();
                str.push(c);
                extend_with_digits(source, &mut str);
                if source.peek() == Some(&'.') {
                    if let Some(c) = source.peek_2() {
                        if c.is_ascii_digit() {
                            str.push(source.next().unwrap());
                            extend_with_digits(source, &mut str);
                        }
                    }
                }
                Some(Token::new(
                    Number(str.parse::<f64>().unwrap()),
                    consume_span(start, source.get_location()),
                ))
            }

            // Identifiers
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut str = std::string::String::new();
                str.push(c);
                loop {
                    match source.peek() {
                        Some(c) if c.is_ascii_alphanumeric() || *c == '_' => {
                            str.push(source.next().unwrap())
                        }
                        _ => break,
                    }
                }
                Some(Token::new(
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
                        _ => Identifier(str),
                    },
                    consume_span(start, source.get_location()),
                ))
            }

            // Errors
            c => Some(Token::new(
                Invalid(ScanningError::InvalidCharacter(
                    c,
                    CodeSpan::new(Location::new(start.line, start.char), source.get_location()),
                )),
                CodeSpan::new(Location::new(start.line, start.char), source.get_location()),
            )),
        };
    }
    return None;
}

/// Scans every token in the given source and returns either the first error or
/// a vector of all scanner tokens.
#[cfg(test)]
pub fn scan_all(code: &str) -> Vec<Token> {
    let mut source = LocationTrackingIterator::new(code.chars());
    let mut vec = Vec::new();
    let mut loc = Location::start();
    while let Some(token) = scan(&mut source, &mut loc) {
        vec.push(token);
    }
    vec
}

#[cfg(test)]
pub fn to_string(vec: Vec<Token>) -> std::string::String {
    let mut s = std::string::String::new();
    for token in vec {
        s.extend(format!("{:?}", token).chars());
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::scanning::scan_all;

    fn assert_equals(to_be_parsed: &str, expected: &str) {
        let parsed = scan_all(to_be_parsed);
        let s = super::to_string(parsed);
        assert_eq!(s, expected);
    }

    #[test]
    fn empty() {
        let code = "";
        let expected = "";
        assert_equals(code, expected);
    }

    #[test]
    fn one_operator() {
        let code = "+";
        let expected = "\
        [1,0]-[1,1] Plus\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn two_char_operator() {
        let code = "!=";
        let expected = "\
        [1,0]-[1,2] BangEqual\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn bang_equal_equal() {
        let code = "!==";
        let expected = "[1,0]-[1,2] BangEqual\n\
        [1,2]-[1,3] Equal\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn all_operators() {
        let code = "(){},.-+;*!!====/<<=>>=";
        let expected = "\
        [1,0]-[1,1] LeftParen\n\
        [1,1]-[1,2] RightParen\n\
        [1,2]-[1,3] LeftBrace\n\
        [1,3]-[1,4] RightBrace\n\
        [1,4]-[1,5] Comma\n\
        [1,5]-[1,6] Dot\n\
        [1,6]-[1,7] Minus\n\
        [1,7]-[1,8] Plus\n\
        [1,8]-[1,9] Semicolon\n\
        [1,9]-[1,10] Star\n\
        [1,10]-[1,11] Bang\n\
        [1,11]-[1,13] BangEqual\n\
        [1,13]-[1,15] EqualEqual\n\
        [1,15]-[1,16] Equal\n\
        [1,16]-[1,17] Slash\n\
        [1,17]-[1,18] Less\n\
        [1,18]-[1,20] LessEqual\n\
        [1,20]-[1,21] Greater\n\
        [1,21]-[1,23] GreaterEqual\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_whitespace() {
        let code = "\t \n";
        let expected = "";
        assert_equals(code, expected);
    }

    #[test]
    fn whitespace_between_ops() {
        let code = "! =\n=\t=";
        let expected = "\
        [1,0]-[1,1] Bang\n\
        [1,2]-[1,3] Equal\n\
        [2,0]-[2,1] Equal\n\
        [2,2]-[2,3] Equal\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_comment() {
        let code = "// This is but a comment";
        let expected = "\
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
        [1,0]-[1,1] LeftParen\n\
        [2,0]-[2,1] RightParen\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn only_string() {
        let code = r#""a string""#;
        let expected = "\
        [1,0]-[1,10] String(\"a string\")\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn empty_string() {
        let code = r#""""#;
        let expected = "\
        [1,0]-[1,2] String(\"\")\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn string_and_operator() {
        let code = r#"""+"#;
        let expected = "\
        [1,0]-[1,2] String(\"\")\n\
        [1,2]-[1,3] Plus\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn unterminated_string() {
        let code = r#""i swear i am compl"#;
        let expected = "\
        [1,0]-[1,19] Invalid(UnterminatedString([1,0]-[1,19]))\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn integer() {
        let code = "0";
        let expected = "\
        [1,0]-[1,1] Number(0.0)\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn float() {
        let code = "1.0";
        let expected = "\
        [1,0]-[1,3] Number(1.0)\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn invalid_floats() {
        let code = ".1 1.";
        let expected = "\
        [1,0]-[1,1] Dot\n\
        [1,1]-[1,2] Number(1.0)\n\
        [1,3]-[1,4] Number(1.0)\n\
        [1,4]-[1,5] Dot\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn number_literal_method_call() {
        let code = "\
        1.square()\n\
        -2.0.abs()\n\
        ";
        let expected = "\
        [1,0]-[1,1] Number(1.0)\n\
        [1,1]-[1,2] Dot\n\
        [1,2]-[1,8] Identifier(\"square\")\n\
        [1,8]-[1,9] LeftParen\n\
        [1,9]-[1,10] RightParen\n\
        [2,0]-[2,1] Minus\n\
        [2,1]-[2,4] Number(2.0)\n\
        [2,4]-[2,5] Dot\n\
        [2,5]-[2,8] Identifier(\"abs\")\n\
        [2,8]-[2,9] LeftParen\n\
        [2,9]-[2,10] RightParen\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn identifier() {
        let code = "Bond";
        let expected = "\
        [1,0]-[1,4] Identifier(\"Bond\")\n\
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
        [1,0]-[1,4] Identifier(\"Bond\")\n\
        [1,5]-[1,10] Identifier(\"James\")\n\
        [2,0]-[2,4] Identifier(\"b0nd\")\n\
        [2,5]-[2,9] Identifier(\"_007\")\n\
        [3,0]-[3,1] Identifier(\"b\")\n\
        [3,2]-[3,3] Identifier(\"_\")\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn reserved_word() {
        let code = "if";
        let expected = "\
        [1,0]-[1,2] If\n\
        ";
        assert_equals(code, expected);
    }

    #[test]
    fn maximal_munch() {
        let code = "ifor";
        let expected = "\
        [1,0]-[1,4] Identifier(\"ifor\")\n\
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
        let expected = "[1,0]-[1,3] And\n\
        [2,0]-[2,5] Class\n\
        [3,0]-[3,4] Else\n\
        [4,0]-[4,5] False\n\
        [5,0]-[5,3] For\n\
        [6,0]-[6,3] Fun\n\
        [7,0]-[7,2] If\n\
        [8,0]-[8,3] Nil\n\
        [9,0]-[9,2] Or\n\
        [10,0]-[10,5] Print\n\
        [11,0]-[11,6] Return\n\
        [12,0]-[12,5] Super\n\
        [13,0]-[13,4] This\n\
        [14,0]-[14,4] True\n\
        [15,0]-[15,3] Var\n\
        [16,0]-[16,5] While\n\
        ";
        assert_equals(code, expected);
    }
}
