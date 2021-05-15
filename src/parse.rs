use std::ops::Range;

use super::{
    call::Call,
    expression::Expression,
    immediate::Immediate,
    lex::{lex, Token},
};

#[derive(Debug)]
pub struct ParseError;

/*
enum Symbol {
    Expression,
    Immediate,
    LeftParenthesis,
    UnaryPrimitive,
    RightParenthesis,
    Null,
    Boolean,
    Character,
    Integer
}
*/

pub fn parse(source: &str) -> Result<Expression, ParseError> {
    let mut tokens = lex(source);

    expression(&mut tokens)
}

// This is shitty code
fn expression<'a>(
    tokens: &mut impl Iterator<Item = (Token<'a>, Range<usize>)>,
) -> Result<Expression, ParseError> {
    use Token::*;

    if let Some((token, _)) = tokens.next() {
        match token {
            LeftParenthesis => {
                if let Some((Symbol(symbol), _)) = tokens.next() {
                    if let Ok(primitive) = symbol.parse() {
                        let rest = expression(tokens)?;
                        if let Some((RightParenthesis, _)) = tokens.next() {
                            Ok(Expression::Call(Call::new(primitive, rest)))
                        } else {
                            Err(ParseError)
                        }
                    } else {
                        Err(ParseError)
                    }
                } else {
                    Err(ParseError)
                }
            }
            Null => Ok(Expression::Immediate(Immediate::Null)),
            Boolean(boolean) => Ok(Expression::Immediate(Immediate::Boolean(boolean))),
            Character(character) => Ok(Expression::Immediate(Immediate::Character(character))),
            Integer(integer) => Ok(Expression::Immediate(Immediate::Integer(integer))),
            _ => Err(ParseError),
        }
    } else {
        Err(ParseError)
    }
}
