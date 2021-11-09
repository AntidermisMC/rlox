mod conversions;
pub mod expressions;
pub mod statements;

pub enum LiteralValue {
    StringLiteral(String),
    NumberLiteral(f64),
    True,
    False,
    Nil,
}
