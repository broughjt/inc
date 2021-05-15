use std::{fmt, str::FromStr};

use super::{
    call::Call,
    immediate::Immediate,
    parse::{parse, ParseError},
};

#[derive(Debug)]
pub enum Expression {
    Call(Call),
    Immediate(Immediate),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;

        match self {
            Call(call) => write!(f, "({} {})", call.primitive, call.expression),
            Immediate(immediate) => write!(f, "{}", immediate),
        }
    }
}

impl FromStr for Expression {
    type Err = ParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse(source)
    }
}
