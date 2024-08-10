use anyhow::Result;
use assert_cmd::Command;
use std::fs;

#[test]
fn hello() {

    let _ = env_logger::builder().is_test(false).try_init();
}

#[test]
fn dies_no_args() -> Result<()>{
    let mut cmd = Command::cargo_bin("echor_2")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor_2").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() -> Result<()>{
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()>{
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()>{
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()>{
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor_2")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}