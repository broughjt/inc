use std::{fmt, str::FromStr};

use super::{
    lex::lex,
    parse::{parse, ParseError},
};

pub mod call;
pub mod immediate;

use call::Call;
use immediate::Immediate;

#[derive(Debug)]
pub enum Expression {
    Call(Call),
    Immediate(Immediate),
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Expression::Call(Call {
                    primitive: p1,
                    expression: e1,
                }),
                Expression::Call(Call {
                    primitive: p2,
                    expression: e2,
                }),
            ) => p1 == p2 && e1 == e2,
            (Expression::Immediate(i1), Expression::Immediate(i2)) => i1 == i2,
            _ => false,
        }
    }
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
        parse(&mut lex(source))
    }
}
