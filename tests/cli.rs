use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // enable run binary in target directory
    let mut cmd = Command::cargo_bin("echor")?;
    // ensure program fail when run without arguments
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).unwrap().assert().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    run(vec!["Hello there"], outfile)
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    run(vec!["Hello", "there"], outfile)
}

#[test]
fn hello1_no_newline() -> TestResult {
    let outfile = "tests/expected/hello1.n.txt";
    // with 2 spaces!
    run(vec!["Hello  there", "-n"], outfile)
}

#[test]
fn hello2_no_newline() -> TestResult {
    let outfile = "tests/expected/hello2.n.txt";
    run(vec!["-n", "Hello", "there"], outfile)
}
