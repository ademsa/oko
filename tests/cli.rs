use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::str::contains;
use std::fs::remove_file;

#[test]
fn test_config_file_not_found() {
    confy::get_configuration_file_path("minigrep", "local")
        .and_then(|file_path| {
            remove_file(file_path).unwrap();
            Ok(())
        })
        .unwrap();

    let file = assert_fs::NamedTempFile::new("content-1.txt").unwrap();
    file.write_str("hello\nmy\nfriend").unwrap();

    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg("my").arg(file.path());

    cmd.assert()
        .success()
        .stdout(contains("2: \u{1b}[32mmy\u{1b}[39m\n"));

    file.close().unwrap();
}

#[test]
fn test_content_file_not_found() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg("my").arg("content-2.txt");

    cmd.assert()
        .failure()
        .code(101)
        .stderr(contains("Error reading file content-2.txt".to_string()));
}

#[test]
fn test_find() {
    let file = assert_fs::NamedTempFile::new("content-1.txt").unwrap();
    file.write_str("hello\nmy\nfriend").unwrap();

    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg("my").arg(file.path());

    cmd.assert()
        .success()
        .stdout(contains("2: \u{1b}[32mmy\u{1b}[39m\n"));

    file.close().unwrap();
}

#[test]
fn test_find_regex() {
    let file = assert_fs::NamedTempFile::new("content-1.txt").unwrap();
    file.write_str("Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.")
        .unwrap();

    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg(r"\byou\b").arg("-r").arg(file.path());

    cmd.assert().success().stdout(contains(
        "2: \u{1b}[32myou\u{1b}[39m\n3: \u{1b}[32myou\u{1b}[39m\n",
    ));

    file.close().unwrap();
}

#[test]
fn test_count() {
    let file = assert_fs::NamedTempFile::new("content-1.txt").unwrap();
    file.write_str("hello\nmy\nfriend").unwrap();

    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg("my").arg(file.path()).arg("-c");

    cmd.assert().success().stdout(contains("1"));

    file.close().unwrap();
}

#[test]
fn test_count_regex() {
    let file = assert_fs::NamedTempFile::new("content-1.txt").unwrap();
    file.write_str("Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.")
        .unwrap();

    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    cmd.arg(r"\byou\b").arg("-c").arg("-r").arg(file.path());

    cmd.assert().success().stdout(contains("2"));

    file.close().unwrap();
}
