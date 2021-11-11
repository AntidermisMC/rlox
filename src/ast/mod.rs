mod conversions;
pub mod declarations;
pub mod expressions;
pub mod statements;

#[derive(PartialEq)]
pub enum LiteralValue {
    StringLiteral(String),
    NumberLiteral(f64),
    True,
    False,
    Nil,
}
