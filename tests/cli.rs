use anyhow::Result;
use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::str::contains;
use std::fs::remove_file;

#[test]
fn config_file_not_found() -> Result<()> {
    confy::get_configuration_file_path("minigrep", "local").and_then(|file_path| {
        remove_file(file_path).unwrap();
        Ok(())
    })?;

    let file = assert_fs::NamedTempFile::new("content-1.txt")?;
    file.write_str("hello\nmy\nfriend")?;

    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("my").arg(file.path());

    cmd.assert()
        .success()
        .stdout(contains("2: \u{1b}[32mmy\u{1b}[39m\n"));

    file.close().unwrap();

    Ok(())
}

#[test]
fn find_matches() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("content-1.txt")?;
    file.write_str("hello\nmy\nfriend")?;

    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("my").arg(file.path());

    cmd.assert()
        .success()
        .stdout(contains("2: \u{1b}[32mmy\u{1b}[39m\n"));

    file.close().unwrap();

    Ok(())
}

#[test]
fn content_file_not_found() -> Result<()> {
    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("my").arg("content-2.txt");

    cmd.assert()
        .failure()
        .code(101)
        .stderr(contains("Error reading file content-2.txt".to_string()));

    Ok(())
}
