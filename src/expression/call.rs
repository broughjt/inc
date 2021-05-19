use std::{fmt, str::FromStr};

use super::Expression;

#[derive(Debug, PartialEq)]
pub struct Call {
    pub primitive: UnaryPrimitive,
    pub expression: Box<Expression>,
}

impl Call {
    pub fn new(primitive: UnaryPrimitive, expression: Expression) -> Self {
        Self {
            primitive,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryPrimitive {
    FxAdd1,
    FxSub1,
    FxLogNot,
    CharToFixnum,
    FixnumToChar,
    FixnumIsZero,
    IsNull,
    Not,
    IsFixnum,
    IsBoolean,
    IsCharacter,
}

pub struct ParseUnaryPrimitiveError;

impl FromStr for UnaryPrimitive {
    type Err = ParseUnaryPrimitiveError;

    fn from_str(symbol: &str) -> Result<Self, Self::Err> {
        use UnaryPrimitive::*;

        match symbol {
            "fxadd1" => Ok(FxAdd1),
            "fxsub1" => Ok(FxSub1),
            "fxlognot" => Ok(FxLogNot),
            "char->fixnum" => Ok(CharToFixnum),
            "fixnum->char" => Ok(FixnumToChar),
            "fxzero?" => Ok(FixnumIsZero),
            "null?" => Ok(IsNull),
            "not" => Ok(Not),
            "fixnum?" => Ok(IsFixnum),
            "boolean?" => Ok(IsBoolean),
            "char?" => Ok(IsCharacter),
            _ => Err(ParseUnaryPrimitiveError),
        }
    }
}

impl fmt::Display for UnaryPrimitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnaryPrimitive::*;

        let primitive = match self {
            FxAdd1 => "fxadd1",
            FxSub1 => "fxsub1",
            FxLogNot => "fxlognot",
            CharToFixnum => "char->fixnum",
            FixnumToChar => "fixnum->char",
            FixnumIsZero => "fxzero?",
            IsNull => "null?",
            Not => "not",
            IsFixnum => "fixnum?",
            IsBoolean => "boolean?",
            IsCharacter => "char?",
        };

        f.write_str(primitive)
    }
}
