use std::ops::Range;

use super::{
    expression::{
        call::{Call, UnaryPrimitive},
        immediate::Immediate,
        Expression,
    },
    lex::Token,
};

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
enum State {
    Zero,
    One(Expression),
    Two,
    Three(Immediate),
    Four(UnaryPrimitive),
    Five(Expression),
    Six,
}

#[derive(Debug)]
enum Production {
    Immediate,
    Call,
}

#[derive(Debug)]
enum Action {
    Shift(State),
    Reduce(Production),
    Accept,
    Error,
}

pub fn parse<'a>(
    tokens: &mut impl Iterator<Item = (Token<'a>, Range<usize>)>,
) -> Result<Expression, ParseError> {
    let mut stack = vec![State::Zero];
    let mut token = tokens.next().map(|(token, _)| token);

    loop {
        if let Some(state) = stack.last() {
            match action(state, token.as_ref()) {
                Action::Shift(state) => {
                    stack.push(state);
                    token = tokens.next().map(|(token, _)| token);
                }
                Action::Reduce(production) => match production {
                    Production::Immediate => {
                        if let Some(State::Three(immediate)) = stack.pop() {
                            match stack.last() {
                                Some(State::Zero) => {
                                    stack.push(State::One(Expression::Immediate(immediate)))
                                }
                                Some(State::Four(_)) => {
                                    stack.push(State::Five(Expression::Immediate(immediate)))
                                }
                                _ => (),
                            };
                        } else {
                            break Err(ParseError);
                        }
                    }
                    Production::Call => {
                        if let (
                            Some(State::Six),
                            Some(State::Five(expression)),
                            Some(State::Four(primitive)),
                            Some(State::Two),
                        ) = (stack.pop(), stack.pop(), stack.pop(), stack.pop())
                        {
                            match stack.last() {
                                Some(State::Zero) => stack.push(State::One(Expression::Call(
                                    Call::new(primitive, expression),
                                ))),
                                Some(State::Four(_)) => stack.push(State::Five(Expression::Call(
                                    Call::new(primitive, expression),
                                ))),
                                _ => (),
                            };
                        } else {
                            break Err(ParseError);
                        }
                    }
                },
                Action::Accept => {
                    break match stack.pop() {
                        Some(State::One(expression)) => Ok(expression),
                        _ => Err(ParseError),
                    }
                }
                Action::Error => break Err(ParseError),
            }
        } else {
            break Err(ParseError);
        }
    }
}

fn action(state: &State, token: Option<&Token>) -> Action {
    match (state, token) {
        (State::Zero, Some(Token::LeftParenthesis)) => Action::Shift(State::Two),
        (State::Zero, Some(Token::Null)) => Action::Shift(State::Three(Immediate::Null)),
        (State::Zero, Some(Token::Boolean(boolean))) => {
            Action::Shift(State::Three(Immediate::Boolean(*boolean)))
        }
        (State::Zero, Some(Token::Character(character))) => {
            Action::Shift(State::Three(Immediate::Character(*character)))
        }
        (State::Zero, Some(Token::Integer(integer))) => {
            Action::Shift(State::Three(Immediate::Integer(*integer)))
        }
        (State::One(_), None) => Action::Accept,
        (State::Two, Some(Token::Symbol(symbol))) => {
            if let Ok(primitive) = symbol.parse() {
                Action::Shift(State::Four(primitive))
            } else {
                Action::Error
            }
        }
        (State::Three(_), Some(Token::RightParenthesis)) | (State::Three(_), None) => {
            Action::Reduce(Production::Immediate)
        }
        (State::Four(_), Some(Token::LeftParenthesis)) => Action::Shift(State::Two),
        (State::Four(_), Some(Token::Null)) => Action::Shift(State::Three(Immediate::Null)),
        (State::Four(_), Some(Token::Boolean(boolean))) => {
            Action::Shift(State::Three(Immediate::Boolean(*boolean)))
        }
        (State::Four(_), Some(Token::Character(character))) => {
            Action::Shift(State::Three(Immediate::Character(*character)))
        }
        (State::Four(_), Some(Token::Integer(integer))) => {
            Action::Shift(State::Three(Immediate::Integer(*integer)))
        }
        (State::Five(_), Some(Token::RightParenthesis)) => Action::Shift(State::Six),
        (State::Six, Some(Token::RightParenthesis)) | (State::Six, None) => {
            Action::Reduce(Production::Call)
        }
        _ => Action::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let cases = [
            ("()", Expression::Immediate(Immediate::Null)),
            ("#t", Expression::Immediate(Immediate::Boolean(true))),
            ("#f", Expression::Immediate(Immediate::Boolean(false))),
            (r"#\a", Expression::Immediate(Immediate::Character(b'a'))),
            (r"#\z", Expression::Immediate(Immediate::Character(b'z'))),
            ("5", Expression::Immediate(Immediate::Integer(5))),
            ("-600", Expression::Immediate(Immediate::Integer(-600))),
            (
                "(fxadd1 60)",
                Expression::Call(Call::new(
                    UnaryPrimitive::FxAdd1,
                    Expression::Immediate(Immediate::Integer(60)),
                )),
            ),
            (
                "(fxsub1 5)",
                Expression::Call(Call::new(
                    UnaryPrimitive::FxSub1,
                    Expression::Immediate(Immediate::Integer(5)),
                )),
            ),
            (
                "(fxlognot 1)",
                Expression::Call(Call::new(
                    UnaryPrimitive::FxLogNot,
                    Expression::Immediate(Immediate::Integer(1)),
                )),
            ),
            (
                r"(char->fixnum #\a)",
                Expression::Call(Call::new(
                    UnaryPrimitive::CharToFixnum,
                    Expression::Immediate(Immediate::Character(b'a')),
                )),
            ),
            (
                "(fixnum->char 55)",
                Expression::Call(Call::new(
                    UnaryPrimitive::FixnumToChar,
                    Expression::Immediate(Immediate::Integer(55)),
                )),
            ),
            (
                "(fxzero? 0)",
                Expression::Call(Call::new(
                    UnaryPrimitive::FixnumIsZero,
                    Expression::Immediate(Immediate::Integer(0)),
                )),
            ),
            (
                "(null? ())",
                Expression::Call(Call::new(
                    UnaryPrimitive::IsNull,
                    Expression::Immediate(Immediate::Null),
                )),
            ),
            (
                "(not #t)",
                Expression::Call(Call::new(
                    UnaryPrimitive::Not,
                    Expression::Immediate(Immediate::Boolean(true)),
                )),
            ),
            (
                "(fixnum? 6)",
                Expression::Call(Call::new(
                    UnaryPrimitive::IsFixnum,
                    Expression::Immediate(Immediate::Integer(6)),
                )),
            ),
            (
                "(boolean? #f)",
                Expression::Call(Call::new(
                    UnaryPrimitive::IsBoolean,
                    Expression::Immediate(Immediate::Boolean(false)),
                )),
            ),
            (
                r"(char? #\z)",
                Expression::Call(Call::new(
                    UnaryPrimitive::IsCharacter,
                    Expression::Immediate(Immediate::Character(b'z')),
                )),
            ),
            (
                "(char? (fixnum->char (fxadd1 55)))",
                Expression::Call(Call::new(
                    UnaryPrimitive::IsCharacter,
                    Expression::Call(Call::new(
                        UnaryPrimitive::FixnumToChar,
                        Expression::Call(Call::new(
                            UnaryPrimitive::FxAdd1,
                            Expression::Immediate(Immediate::Integer(55)),
                        )),
                    )),
                )),
            ),
        ];

        for (source, expected) in &cases {
            assert_eq!(source.parse::<Expression>().unwrap(), *expected);
        }
    }

    #[test]
    fn failure() {
        let cases = [
            "(",
            "(fxadd1)",
            "(5 #t)",
            "(fxadd1 2) 5",
            "#f 45",
            "",
            " ",
            r"(hello #\a)",
        ];

        for source in &cases {
            assert!(source.parse::<Expression>().is_err());
        }
    }
}
