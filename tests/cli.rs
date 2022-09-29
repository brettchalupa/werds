use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn single_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("werds")?;

    cmd.arg("tests/fixtures/haiku.txt");
    cmd.assert().success().stdout(String::from("7\n"));

    Ok(())
}

#[test]
fn medium_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("werds")?;

    cmd.arg("tests/fixtures/medium.txt");
    cmd.assert().success().stdout(String::from("8\n"));

    Ok(())
}

#[test]
fn long_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("werds")?;

    cmd.arg("tests/fixtures/long.txt");
    cmd.assert().success().stdout(String::from("204\n"));

    Ok(())
}

#[test]
fn multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("werds")?;

    cmd.arg("tests/fixtures/haiku.txt")
        .arg("tests/fixtures/medium.txt")
        .arg("tests/fixtures/long.txt");
    cmd.assert().success().stdout(String::from("tests/fixtures/haiku.txt: 7\ntests/fixtures/medium.txt: 8\ntests/fixtures/long.txt: 204\ntotal: 219\n"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("werds")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

// TODO: directory error
// wc: tests/fixtures/: read: Is a directory
