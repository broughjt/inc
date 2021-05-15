use std::fmt::Write;

use super::{
    call::{Call, UnaryPrimitive},
    expression::Expression,
    immediate::{
        Immediate, BOOLEAN_BIT, BOOLEAN_FALSE, BOOLEAN_MASK, CHARACTER_MASK, CHARACTER_SHIFT,
        CHARACTER_TAG, INTEGER_MASK, INTEGER_SHIFT, INTEGER_TAG, NULL,
    },
    parse::ParseError,
};

#[derive(Debug)]
pub enum CompilationError {
    LexicalError,
    ParseError,
    Other,
}

impl From<ParseError> for CompilationError {
    fn from(_value: ParseError) -> Self {
        CompilationError::ParseError
    }
}

pub fn compile(expression: Expression) -> Result<String, CompilationError> {
    let mut output = String::with_capacity(32); // TODO

    write!(
        output,
        ".globl _scheme_entry\n\
        _scheme_entry:\n"
    )
    .unwrap();
    emit_expression(&mut output, expression)?;
    write!(output, "\tret\n").unwrap();

    Ok(output)
}

pub fn run(source: &str) -> Result<String, CompilationError> {
    source
        .parse()
        .map_err(CompilationError::from)
        .and_then(compile)
}

fn emit_expression(output: &mut String, expression: Expression) -> Result<(), CompilationError> {
    match expression {
        Expression::Call(Call {
            primitive,
            expression,
        }) => {
            use UnaryPrimitive::*;

            emit_expression(output, *expression)?;

            let one = i32::from(Immediate::Integer(1));

            match primitive {
                FxAdd1 => write!(output, "\taddl ${}, %eax\n", one).unwrap(),
                FxSub1 => write!(output, "\tsubl ${}, %eax\n", one).unwrap(),
                FxLogNot => write!(
                    output,
                    "\tshr ${}, %eax\n\
                    \tnot %eax\n
                    \tshl ${}, %eax\n",
                    INTEGER_SHIFT, INTEGER_SHIFT
                )
                .unwrap(),
                FixnumToChar => write!(
                    // TODO
                    output,
                    "\tshll ${}, %eax\n\
                    \torl ${}, %eax\n",
                    CHARACTER_SHIFT - INTEGER_SHIFT,
                    CHARACTER_TAG
                )
                .unwrap(),
                CharToFixnum => write!(
                    output,
                    "\tshrl ${}, %eax\n",
                    CHARACTER_SHIFT - INTEGER_SHIFT
                )
                .unwrap(), // TODO
                FixnumIsZero => {
                    write!(output, "\tcmp ${}, %al\n", INTEGER_TAG).unwrap();
                    emit_comparison(output);
                }
                IsNull => {
                    write!(output, "\tcmp ${}, %al\n", NULL).unwrap(); // TODO
                    emit_comparison(output);
                }
                Not => {
                    write!(output, "\tcmp ${}, %al\n", BOOLEAN_FALSE).unwrap();
                    emit_comparison(output);
                }
                IsFixnum => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        INTEGER_MASK, // TODO
                        INTEGER_TAG
                    )
                    .unwrap();
                    emit_comparison(output);
                }
                IsBoolean => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        BOOLEAN_MASK, // TODO
                        BOOLEAN_FALSE
                    )
                    .unwrap();
                    emit_comparison(output);
                }
                IsCharacter => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        CHARACTER_MASK, // TODO
                        CHARACTER_TAG
                    )
                    .unwrap();
                    emit_comparison(output);
                }
            }
        }
        Expression::Immediate(immediate) => {
            write!(output, "\tmovl ${}, %eax\n", i32::from(immediate)).unwrap()
        }
    }

    Ok(())
}

fn emit_comparison(output: &mut String) {
    write!(
        output,
        "\tsete %al\n\
        \tmovzbl %al, %eax\n\
        \tsal ${}, %al\n\
        \tor ${}, %al\n",
        BOOLEAN_BIT, BOOLEAN_FALSE
    )
    .unwrap()
}
