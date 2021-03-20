use super::expression::{Expression, ParseError};

pub fn compile(x: Expression) -> String {
    let Expression::Immediate(x) = x;

    format!(
        ".globl _scheme_entry\n\
        _scheme_entry:\n\
        \tmovl ${}, %eax\n\
        \tret\n",
        i32::from(x)
    )
}

pub fn run(source: &str) -> Result<String, ParseError> {
    source.parse().map(compile)
}