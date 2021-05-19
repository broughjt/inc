use std::fmt;

pub const BOOLEAN_BIT: i32 = 6;
pub const BOOLEAN_MASK: i32 = 0xbf;
pub const BOOLEAN_TRUE: i32 = 0x6f;
pub const BOOLEAN_FALSE: i32 = 0x2f;

pub const INTEGER_MASK: i32 = 0x03;
pub const INTEGER_TAG: i32 = 0x00;
pub const INTEGER_SHIFT: i32 = 2;

pub const CHARACTER_MASK: i32 = 0x3f;
pub const CHARACTER_TAG: i32 = 0x0f;
pub const CHARACTER_SHIFT: i32 = 8;

pub const NULL: i32 = 0x3f;

#[derive(Debug, PartialEq)]
pub enum Immediate {
    Null,
    Boolean(bool),
    Character(u8),
    Integer(i32),
}

impl From<Immediate> for i32 {
    fn from(immediate: Immediate) -> Self {
        use Immediate::*;

        match immediate {
            Boolean(true) => BOOLEAN_TRUE,
            Boolean(false) => BOOLEAN_FALSE,
            Integer(n) => n << INTEGER_SHIFT,
            Character(c) => (i32::from(c) << CHARACTER_SHIFT) | CHARACTER_TAG,
            Null => NULL,
        }
    }
}

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Immediate::*;

        match self {
            Null => f.write_str("()"),
            Boolean(true) => f.write_str("#t"),
            Boolean(false) => f.write_str("#f"),
            Character(b'\n') => f.write_str("#\\newline"),
            Character(b'\r') => f.write_str("#\\return"),
            Character(b' ') => f.write_str("#\\space"),
            Character(b'\t') => f.write_str("#\\tab"),
            Character(character) => write!(f, "#\\{}", *character as char),
            Integer(integer) => write!(f, "{}", integer),
        }
    }
}
