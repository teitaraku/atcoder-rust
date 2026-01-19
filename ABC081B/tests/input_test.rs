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
    let input = "3
    8 12 40
    ";
    assert_eq!(run_with_input(input), "2");
}

#[test]
fn example2() {
    let input = "4
    5 6 8 10
    ";
    assert_eq!(run_with_input(input), "0");
}

#[test]
fn example3() {
    let input = "6
382253568 723152896 37802240 379425024 404894720 471526144
";
    assert_eq!(run_with_input(input), "8");
}
