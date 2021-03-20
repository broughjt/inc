use std::{fs, path::Path, process::Command};
use inc::compile::run;
use rand::random;

fn test(input: &str, output: &str) {
    let directory = format!("/tmp/inc-{}", random::<u32>());
    let directory = Path::new(&directory);
    let object = directory.join("test.s");
    let binary = directory.join("test");

    fs::create_dir_all(&directory).unwrap();
    fs::write(&object, run(input).unwrap()).unwrap();

    let status = Command::new("clang")
        .arg(&object)
        .arg("src/runtime.c")
        .arg("-o")
        .arg(&binary)
        .status()
        .unwrap();

    assert!(status.success());

    let result = Command::new(&binary).output().unwrap();

    assert!(result.status.success());
    assert_eq!(output, String::from_utf8(result.stdout).unwrap().trim());

    fs::remove_dir_all(&directory).unwrap();
}

#[test]
fn integers() {
    for n in &[0, 1, -1, 10, -10, 2736, -2736, 536_870_911, -536_870_912] {
        let s = n.to_string();
        test(&s, &s);
    }
}

#[test]
fn immediate_constants() {
    let constants = [
        r"#t",
        r"#f",
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