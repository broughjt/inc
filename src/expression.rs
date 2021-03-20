use std::str::FromStr;
use super::immediate::Immediate;

pub enum Expression {
    Immediate(Immediate)
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedInput
}

fn parse(input: &str) -> Result<Expression, ParseError> {
    match input {
        "#t" => return Ok(Expression::Immediate(Immediate::Boolean(true))),
        "#f" => return Ok(Expression::Immediate(Immediate::Boolean(false))),
        "()" => return Ok(Expression::Immediate(Immediate::Null)),
        _ => ()
    };

    if let Ok(number) = input.parse() {
        Ok(Expression::Immediate(Immediate::Fixnum(number)))
    } else if let Some(character) = input.strip_prefix("#\\") {
        match character {
            "tab" => return Ok(Expression::Immediate(Immediate::Character(b'\t'))),
            "newline" => return Ok(Expression::Immediate(Immediate::Character(b'\n'))),
            "return" => return Ok(Expression::Immediate(Immediate::Character(b'\r'))),
            "space" => return Ok(Expression::Immediate(Immediate::Character(b' '))),
            _ => ()
        }

        if let Some(byte) = character.bytes().next().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation()) {
            Ok(Expression::Immediate(Immediate::Character(byte)))
        } else {
            Err(ParseError::UnexpectedInput)
        }
    } else {
        Err(ParseError::UnexpectedInput)
    }
}

impl FromStr for Expression {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}