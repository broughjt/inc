use inc::compile::compile;
use std::{fs::write, process::Command};

mod temporary_directory;

use temporary_directory::TemporaryDirectory;

fn test(input: &str, expected: &str) {
    let directory = TemporaryDirectory::create().unwrap();
    let object = directory.0.join("test.s");
    let binary = directory.0.join("test");

    let output = compile(input.parse().unwrap()).unwrap();
    write(&object, output).unwrap();

    let status = Command::new("clang")
        .arg(&object)
        .arg("src/runtime.c")
        .arg("-o")
        .arg(&binary)
        .status()
        .unwrap();
    assert!(status.success());

    let result = Command::new(&binary).output().unwrap();
    let actual = String::from_utf8(result.stdout).unwrap();

    assert!(result.status.success());
    assert_eq!(expected, actual.trim());
}

fn cases<'a, I: IntoIterator<Item = &'a (&'a str, &'a str)>>(cases: I) {
    for (input, output) in cases {
        test(input, output);
    }
}

// Step 1: Integers
mod integers {
    use super::*;

    #[test]
    fn integers() {
        for n in &[0, 1, -1, 10, -10, 2736, -2736, 536_870_911, -536_870_912] {
            let s = n.to_string();
            test(&s, &s);
        }
    }
}

// Step 2: Immediate Constants
mod immediate {
    use super::*;

    #[test]
    fn immediate_constants() {
        let constants = [
            "#t",
            "#f",
            "()",
            r"#\tab",
            r"#\newline",
            r"#\return",
            r"#\space",
            r"#\!",
            r"#\#",
            r"#\$",
            r"#\%",
            r"#\&",
            r"#\'",
            r"#\(",
            r"#\)",
            r"#\*",
            r"#\+",
            r"#\,",
            r"#\-",
            r"#\.",
            r"#\/",
            r"#\0",
            r"#\9",
            r"#\:",
            r"#\;",
            r"#\<",
            r"#\=",
            r"#\>",
            r"#\?",
            r"#\@",
            r"#\A",
            r"#\B",
            r"#\Z",
            r"#\(",
            r"#\\",
            r"#\]",
            r"#\^",
            r"#\_",
            r"#\`",
            r"#\a",
            r"#\b",
            r"#\z",
            r"#\{",
            r"#\|",
            r"#\}",
            r"#\~",
        ];

        for c in &constants {
            test(c, c);
        }
    }
}

// Step 2: Unary Primitives
mod unary {
    use super::*;

    #[test]
    fn fxadd1() {
        cases(&[
            (r"(fxadd1 0)", "1"),
            (r"(fxadd1 -1)", "0"),
            (r"(fxadd1 1)", "2"),
            (r"(fxadd1 -100)", "-99"),
            (r"(fxadd1 1000)", "1001"),
            (r"(fxadd1 536870910)", "536870911"),
            (r"(fxadd1 -536870912)", "-536870911"),
            (r"(fxadd1 (fxadd1 0))", "2"),
            (r"(fxadd1 (fxadd1 (fxadd1 (fxadd1 (fxadd1 12)))))", "17"),
        ]);
    }

    #[test]
    fn fxsub1() {
        cases(&[
            (r"(fxsub1 0)", "-1"),
            (r"(fxsub1 -1)", "-2"),
            (r"(fxsub1 1)", "0"),
            (r"(fxsub1 -100)", "-101"),
            (r"(fxsub1 1000)", "999"),
            (r"(fxsub1 536870911)", "536870910"),
            (r"(fxsub1 -536870911)", "-536870912"),
            (r"(fxsub1 (fxsub1 0))", "-2"),
            (
                r"(fxsub1 (fxsub1 (fxsub1 (fxsub1 (fxsub1 (fxsub1 12))))))",
                "6",
            ),
            (r"(fxsub1 (fxadd1 0))", "0"),
        ]);
    }

    #[test]
    fn fxlognot() {
        cases(&[
            (r"(fxlognot 0)", "-1"),
            (r"(fxlognot -1)", "0"),
            (r"(fxlognot 1)", "-2"),
            (r"(fxlognot -2)", "1"),
            (r"(fxlognot 536870911)", "-536870912"),
            (r"(fxlognot (fxlognot 237463))", "237463"),
        ]);
    }

    #[test]
    fn fixnum_and_character_conversions() {
        cases(&[
            (r"(fixnum->char 65)", "#\\A"),
            (r"(fixnum->char 97)", "#\\a"),
            (r"(fixnum->char 122)", "#\\z"),
            (r"(fixnum->char 90)", "#\\Z"),
            (r"(fixnum->char 48)", "#\\0"),
            (r"(fixnum->char 57)", "#\\9"),
            (r"(char->fixnum #\A)", "65"),
            (r"(char->fixnum #\a)", "97"),
            (r"(char->fixnum #\z)", "122"),
            (r"(char->fixnum #\Z)", "90"),
            (r"(char->fixnum #\0)", "48"),
            (r"(char->fixnum #\9)", "57"),
            (r"(char->fixnum (fixnum->char 12))", "12"),
            (r"(fixnum->char (char->fixnum #\x))", "#\\x"),
        ]);
    }

    #[test]
    fn is_fixnum() {
        cases(&[
            (r"(fixnum? 0)", "#t"),
            (r"(fixnum? 1)", "#t"),
            (r"(fixnum? -1)", "#t"),
            (r"(fixnum? 37287)", "#t"),
            (r"(fixnum? -23873)", "#t"),
            (r"(fixnum? 536870911)", "#t"),
            (r"(fixnum? -536870912)", "#t"),
            (r"(fixnum? #t)", "#f"),
            (r"(fixnum? #f)", "#f"),
            (r"(fixnum? ())", "#f"),
            (r"(fixnum? #\Q)", "#f"),
            (r"(fixnum? (fixnum? 12))", "#f"),
            (r"(fixnum? (fixnum? #f))", "#f"),
            (r"(fixnum? (fixnum? #\A))", "#f"),
            (r"(fixnum? (char->fixnum #\r))", "#t"),
            (r"(fixnum? (fixnum->char 12))", "#f"),
        ]);
    }

    #[test]
    fn is_zero() {
        cases(&[
            (r"(fxzero? 1)", "#f"),
            (r"(fxzero? -1)", "#f"),
            (r"(fxzero? 64)", "#f"),
            (r"(fxzero? 960)", "#f"),
            (r"(fxzero? #f)", "#f"),
            (r"(fxzero? #\newline)", "#f"),
            (r"(fxzero? (fxzero? 0))", "#f"),
            (r"(fxzero? 0)", "#t"),
        ])
    }

    #[test]
    fn is_null() {
        cases(&[
            (r"(null? ())", "#t"),
            (r"(null? #f)", "#f"),
            (r"(null? #t)", "#f"),
            (r"(null? (null? ()))", "#f"),
            (r"(null? #\a)", "#f"),
            (r"(null? 0)", "#f"),
            (r"(null? -10)", "#f"),
            (r"(null? 10)", "#f"),
        ]);
    }

    #[test]
    fn is_boolean() {
        cases(&[
            (r"(boolean? #t)", "#t"),
            (r"(boolean? #f)", "#t"),
            (r"(boolean? 0)", "#f"),
            (r"(boolean? 1)", "#f"),
            (r"(boolean? -1)", "#f"),
            (r"(boolean? ())", "#f"),
            (r"(boolean? #\a)", "#f"),
            (r"(boolean? (boolean? 0))", "#t"),
            (r"(boolean? (fixnum? (boolean? 0)))", "#t"),
        ]);
    }

    #[test]
    fn is_char() {
        cases(&[
            (r"(char? #\a)", "#t"),
            (r"(char? #\Z)", "#t"),
            (r"(char? #\newline)", "#t"),
            (r"(char? #t)", "#f"),
            (r"(char? #f)", "#f"),
            (r"(char? ())", "#f"),
            (r"(char? (char? #t))", "#f"),
            (r"(char? 0)", "#f"),
            (r"(char? 23870)", "#f"),
            (r"(char? -23870)", "#f"),
        ])
    }

    #[test]
    fn not() {
        cases(&[
            (r"(not #t)", "#f"),
            (r"(not #f)", "#t"),
            (r"(not 15)", "#f"),
            (r"(not ())", "#f"),
            (r"(not #\A)", "#f"),
            (r"(not (not #t))", "#t"),
            (r"(not (not #f))", "#f"),
            (r"(not (fixnum? 15))", "#f"),
            (r"(not (fixnum? #f))", "#t"),
        ]);
    }
}
