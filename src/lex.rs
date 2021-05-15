use std::ops::Range;

/*
TODO
- I think I would think this code is ugly if I hadn't written it. It's not bad,
  but some things could be reworked
- A single test isn't gonna cut it
*/

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    LeftParenthesis,
    RightParenthesis,
    Null,
    Whitespace,
    Unrecognized,
    Boolean(bool),
    Character(u8),
    Integer(i32),
    Symbol(&'a str),
}

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

struct Lexer<'a> {
    source: &'a str,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, start: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;

        if self.start >= self.source.len() {
            None
        } else {
            let mut state = Start;
            let mut end = self.start;

            for byte in self.source[self.start..].bytes() {
                state = match state {
                    Start => match byte {
                        b'(' => LeftParenthesis,
                        b')' => RightParenthesis,
                        b'#' => Hash,
                        b'+' | b'-' => Sign,
                        _ if byte.is_ascii_digit() => Integer,
                        _ if is_symbol(byte) => Symbol,
                        _ if byte.is_ascii_whitespace() => Whitespace,
                        _ => break,
                    },
                    LeftParenthesis if byte == b')' => Null,
                    Hash => match byte {
                        b'\\' => Slash,
                        b'f' => False,
                        b't' => True,
                        _ => break,
                    },
                    Slash => {
                        let slice = &self.source[end..];

                        if slice.starts_with("newline") {
                            end += 6;
                            Newline
                        } else if slice.starts_with("return") {
                            end += 5;
                            Return
                        } else if slice.starts_with("space") {
                            end += 4;
                            Space
                        } else if slice.starts_with("tab") {
                            end += 2;
                            Tab
                        } else if byte.is_ascii_graphic() {
                            Character
                        } else {
                            break;
                        }
                    }
                    Sign => match byte {
                        _ if byte.is_ascii_digit() => Integer,
                        _ if is_symbol(byte) => Symbol,
                        _ => break,
                    },
                    Integer if byte.is_ascii_digit() => Integer,
                    Symbol if is_symbol(byte) => Symbol,
                    Whitespace if byte.is_ascii_whitespace() => Whitespace,
                    _ => break,
                };
                end += 1;
            }

            let slice = &self.source[self.start..end];
            let token = match state {
                LeftParenthesis => Token::LeftParenthesis,
                RightParenthesis => Token::RightParenthesis,
                Null => Token::Null,
                False => Token::Boolean(false),
                True => Token::Boolean(true),
                Newline => Token::Character(b'\n'),
                Return => Token::Character(b'\r'),
                Space => Token::Character(b' '),
                Tab => Token::Character(b'\t'),
                Character => Token::Character(slice.as_bytes()[2]),
                Integer => {
                    if let Ok(integer) = slice.parse() {
                        Token::Integer(integer)
                    } else {
                        Token::Unrecognized
                    }
                }
                Sign | Symbol => Token::Symbol(slice),
                Whitespace => Token::Whitespace,
                _ => {
                    end += 1;
                    Token::Unrecognized
                }
            };
            let result = (token, self.start..end);
            self.start = end;

            Some(result)
        }
    }
}

pub fn lex<'a>(source: &'a str) -> impl Iterator<Item = (Token<'a>, Range<usize>)> {
    use Token::*;

    Lexer::new(source).filter(|(token, _)| token != &Whitespace)
}

#[cfg(test)]
mod tests {
    #[test]
    fn lexer() {
        use super::{lex, Token::*};

        let source = r#"(let (a #f
                              b #t
                              c #\t
                              d #\tab
                              e 443
                              f foo
                              g ()
                              h [)
                          (+ +227 b c)
                          (- -119 e f)
                          (fixnum? #t))]"#;
        let tokens = vec![
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
            Symbol("h"),
            Unrecognized,
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
            Unrecognized,
        ];

        assert!(lex(source).map(|(token, _)| token).eq(tokens));
    }
}
