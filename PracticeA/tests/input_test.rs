use std::io::Write;
use std::process::{Command, Stdio};

fn run_with_input(input: &str) -> String {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn example1() {
    let input = "1
    2 3
    test
    ";
    assert_eq!(run_with_input(input), "6 test");
}

#[test]
fn example2() {
    let input = "72
    128 256
    myonmyon
    ";
    assert_eq!(run_with_input(input), "456 myonmyon");
}
