pub const TRUE: i32 = 0x6f;
pub const FALSE: i32 = 0x2f;

pub const FIXNUM_MASK: i32 = 0x03;
pub const FIXNUM_TAG: i32 = 0x00;
pub const FIXNUM_SHIFT: i32 = 2;

pub const CHARACTER_MASK: i32 = 0x0f;
pub const CHARACTER_TAG: i32 = 0xf;
pub const CHARACTER_SHIFT: i32 = 8;

pub const NULL: i32 = 0x3f;


#[derive(Debug, PartialEq)]
pub enum Immediate {
    Boolean(bool),
    Fixnum(i32),
    Character(u8),
    Null
}

impl From<Immediate> for i32 {
    fn from(expression: Immediate) -> Self {
        use Immediate::*;

        match expression {
            Boolean(true) => TRUE,
            Boolean(false) => FALSE,
            Fixnum(n) => n << FIXNUM_SHIFT,
            Character(c) => (i32::from(c) << CHARACTER_SHIFT) | CHARACTER_TAG,
            Null => NULL
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedInput
}