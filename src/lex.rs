use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    LeftParenthesis,
    RightParenthesis,
    Null,
    Boolean(bool),
    Character(u8),
    Integer(i32),
    Symbol(&'a str),
    Unrecognized,
}

pub fn lex(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
    Lexer { source, start: 0 }
}

struct Lexer<'a> {
    source: &'a str,
    start: usize,
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    RightParenthesis,
    LeftParenthesis,
    Null,
    Hash,
    Slash,
    Newline,
    Return,
    Space,
    Tab,
    Character,
    False,
    True,
    Sign,
    Integer,
    Symbol,
    Whitespace,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;

        loop {
            if self.start >= self.source.len() {
                break None;
            } else {
                let mut state = Start;
                let mut end = self.start;

                while let Some(next) = transition(&state, &self.source[end..]) {
                    end += match next {
                        Newline => 7,
                        Return => 6,
                        Space => 5,
                        Tab => 3,
                        _ => 1,
                    };
                    state = next;
                }

                if state == Whitespace {
                    self.start = end;
                    continue;
                } else {
                    let start = self.start;
                    let slice = &self.source[start..end];
                    let token = accept(&state, slice);

                    if token == Token::Unrecognized {
                        end += 1;
                    }
                    self.start = end;

                    break Some((token, start..end));
                }
            }
        }
    }
}

const fn is_symbol(byte: u8) -> bool {
    matches!(
        byte,
        b'!'
        | b'#'..=b'&'
        | b'*'
        | b'+'
        | b'-'..=b':'
        | b'<'..=b'Z'
        | b'^'
        | b'_'
        | b'a'..=b'z'
        | b'|'
        | b'~'
    )
}

fn transition(state: &State, source: &str) -> Option<State> {
    use State::*;

    let byte = source.as_bytes().get(0)?;

    match state {
        Start => match byte {
            b'(' => Some(LeftParenthesis),
            b')' => Some(RightParenthesis),
            b'#' => Some(Hash),
            b'+' | b'-' => Some(Sign),
            _ if byte.is_ascii_digit() => Some(Integer),
            _ if is_symbol(*byte) => Some(Symbol),
            _ if byte.is_ascii_whitespace() => Some(Whitespace),
            _ => None,
        },
        LeftParenthesis if *byte == b')' => Some(Null),
        Hash => match byte {
            b'\\' => Some(Slash),
            b'f' => Some(False),
            b't' => Some(True),
            _ => None,
        },
        Slash => {
            if source.starts_with("newline") {
                Some(Newline)
            } else if source.starts_with("return") {
                Some(Return)
            } else if source.starts_with("space") {
                Some(Space)
            } else if source.starts_with("tab") {
                Some(Tab)
            } else if byte.is_ascii_graphic() {
                Some(Character)
            } else {
                None
            }
        }
        Sign => {
            if byte.is_ascii_digit() {
                Some(Integer)
            } else if is_symbol(*byte) {
                Some(Symbol)
            } else {
                None
            }
        }
        Integer if byte.is_ascii_digit() => Some(Integer),
        Symbol if is_symbol(*byte) => Some(Symbol),
        Whitespace if byte.is_ascii_whitespace() => Some(Whitespace),
        _ => None,
    }
}

fn accept<'a>(state: &State, slice: &'a str) -> Token<'a> {
    match state {
        State::LeftParenthesis => Token::LeftParenthesis,
        State::RightParenthesis => Token::RightParenthesis,
        State::Null => Token::Null,
        State::False => Token::Boolean(false),
        State::True => Token::Boolean(true),
        State::Newline => Token::Character(b'\n'),
        State::Return => Token::Character(b'\r'),
        State::Space => Token::Character(b' '),
        State::Tab => Token::Character(b'\t'),
        State::Character => Token::Character(slice.as_bytes()[2]),
        State::Integer => {
            if let Ok(integer) = slice.parse() {
                Token::Integer(integer)
            } else {
                Token::Unrecognized
            }
        }
        State::Sign | State::Symbol => Token::Symbol(slice),
        _ => Token::Unrecognized,
    }
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn immediates() {
        let cases = [
            ("()", Null),
            ("#t", Boolean(true)),
            ("#f", Boolean(false)),
            (r"#\a", Character(b'a')),
            (r"#\z", Character(b'z')),
            (r"#\A", Character(b'A')),
            (r"#\Z", Character(b'Z')),
            (r"#\newline", Character(b'\n')),
            (r"#\return", Character(b'\r')),
            (r"#\space", Character(b' ')),
            (r"#\tab", Character(b'\t')),
            (r"#\[", Character(b'[')),
            (r"#\}", Character(b'}')),
            ("5", Integer(5)),
            ("+128", Integer(128)),
            ("-128", Integer(-128)),
            ("2147483647", Integer(2147483647)),
            ("-2147483648", Integer(-2147483648)),
        ];

        for (source, expected) in &cases {
            let (actual, _) = lex(source).next().unwrap();

            assert_eq!(
                actual, *expected,
                "source = {:?}, expected = {:?}, actual = {:?}",
                source, expected, actual
            );
        }
    }

    #[test]
    fn full() {
        let source = r#"(let (a #f
                              b #t
                              c #\t
                              d #\tab
                              e 443
                              f foo
                              g ())
                          (+ +227 b c)
                          (- -119 e f)
                          (fixnum? #t))"#;
        let expected = vec![
            LeftParenthesis,
            Symbol("let"),
            LeftParenthesis,
            Symbol("a"),
            Boolean(false),
            Symbol("b"),
            Boolean(true),
            Symbol("c"),
            Character(b't'),
            Symbol("d"),
            Character(b'\t'),
            Symbol("e"),
            Integer(443),
            Symbol("f"),
            Symbol("foo"),
            Symbol("g"),
            Null,
            RightParenthesis,
            LeftParenthesis,
            Symbol("+"),
            Integer(227),
            Symbol("b"),
            Symbol("c"),
            RightParenthesis,
            LeftParenthesis,
            Symbol("-"),
            Integer(-119),
            Symbol("e"),
            Symbol("f"),
            RightParenthesis,
            LeftParenthesis,
            Symbol("fixnum?"),
            Boolean(true),
            RightParenthesis,
            RightParenthesis,
        ];
        let actual = lex(source)
            .map(|(token, _)| token)
            .inspect(|token| println!("{:?}", token));

        assert!(actual.eq(expected));
    }

    #[test]
    fn tricky() {
        let cases = [
            (
                "(hello 5)  ",
                vec![
                    (LeftParenthesis, 0..1),
                    (Symbol("hello"), 1..6),
                    (Integer(5), 7..8),
                    (RightParenthesis, 8..9),
                ],
            ),
            ("", vec![]),
            (" \n\t", vec![]),
            ("[[", vec![(Unrecognized, 0..1), (Unrecognized, 1..2)]),
            (
                "(not #c)",
                vec![
                    (LeftParenthesis, 0..1),
                    (Symbol("not"), 1..4),
                    (Unrecognized, 5..7),
                    (RightParenthesis, 7..8),
                ],
            ),
            (
                "(foo bar",
                vec![
                    (LeftParenthesis, 0..1),
                    (Symbol("foo"), 1..4),
                    (Symbol("bar"), 5..8),
                ],
            ),
        ];

        for (source, expected) in &cases {
            let actual: Vec<(Token, Range<usize>)> = lex(source).collect();

            assert_eq!(actual, *expected);
        }
    }
}
