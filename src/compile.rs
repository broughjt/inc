use std::fmt::Write;

use super::{
    expression::{
        call::{Call, UnaryPrimitive},
        immediate::{
            Immediate, BOOLEAN_BIT, BOOLEAN_FALSE, BOOLEAN_MASK, CHARACTER_MASK, CHARACTER_SHIFT,
            CHARACTER_TAG, INTEGER_MASK, INTEGER_SHIFT, INTEGER_TAG, NULL,
        },
        Expression,
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
    let mut output = String::new();

    write!(
        output,
        ".globl _scheme_entry\n\
        _scheme_entry:\n"
    )
    .unwrap();
    emit_expression(&mut output, expression)?;
    writeln!(output, "\tret").unwrap();

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
                FxAdd1 => writeln!(output, "\taddl ${}, %eax", one).unwrap(),
                FxSub1 => writeln!(output, "\tsubl ${}, %eax", one).unwrap(),
                FxLogNot => write!(
                    output,
                    "\tshr ${}, %eax\n\
                    \tnot %eax\n
                    \tshl ${}, %eax\n",
                    INTEGER_SHIFT, INTEGER_SHIFT
                )
                .unwrap(),
                FixnumToChar => write!(
                    output,
                    "\tshll ${}, %eax\n\
                    \torl ${}, %eax\n",
                    CHARACTER_SHIFT - INTEGER_SHIFT,
                    CHARACTER_TAG
                )
                .unwrap(),
                CharToFixnum => {
                    writeln!(output, "\tshrl ${}, %eax", CHARACTER_SHIFT - INTEGER_SHIFT).unwrap()
                }
                FixnumIsZero => {
                    writeln!(output, "\tcmp ${}, %rax", INTEGER_TAG).unwrap();
                    emit_comparison(output);
                }
                IsNull => {
                    writeln!(output, "\tcmp ${}, %al", NULL).unwrap();
                    emit_comparison(output);
                }
                Not => {
                    writeln!(output, "\tcmp ${}, %al", BOOLEAN_FALSE).unwrap();
                    emit_comparison(output);
                }
                IsFixnum => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        INTEGER_MASK, INTEGER_TAG
                    )
                    .unwrap();
                    emit_comparison(output);
                }
                IsBoolean => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        BOOLEAN_MASK, BOOLEAN_FALSE
                    )
                    .unwrap();
                    emit_comparison(output);
                }
                IsCharacter => {
                    write!(
                        output,
                        "\tand ${}, %al\n\
                        \tcmp ${}, %al\n",
                        CHARACTER_MASK, CHARACTER_TAG
                    )
                    .unwrap();
                    emit_comparison(output);
                }
            }
        }
        Expression::Immediate(immediate) => {
            writeln!(output, "\tmovl ${}, %eax", i32::from(immediate)).unwrap()
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
