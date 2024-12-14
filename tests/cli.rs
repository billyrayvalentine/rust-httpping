use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_hello() {
    assert!(true);
}

#[test]
fn test_args_none() {
    let mut cmd = Command::cargo_bin("http-ping").unwrap();
    cmd.assert().failure().code(2);
}

#[test]
fn test_args_destination_garbage() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("httpbin.org")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::starts_with("Error parsing destination:"));
}

#[test]
fn test_args_destination_not_http_or_https() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("ftp://httpbin.org")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::starts_with(
            "Error parsing destination: \"Scheme must be http or https\"",
        ));
}

#[test]
fn test_args_destination_valid() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("http://httpbin.org").arg("-c1").assert().success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("https://httpbin.org").arg("-c1").assert().success();
}
