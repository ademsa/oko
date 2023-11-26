use anyhow::Result;
use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::str::contains;

#[test]
fn cannot_read_file() -> Result<()> {
    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("my").arg("content-2.txt");
    cmd.assert()
        .failure()
        .stderr(contains("Error reading file"));

    Ok(())
}

#[test]
fn find_matches() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("content-1.txt")?;
    file.write_str("hello\nmy\nfriend")?;

    let mut cmd = Command::cargo_bin("minigrep")?;

    cmd.arg("my").arg(file.path());
    cmd.assert().success().stdout(contains("2: my\n"));

    file.close().unwrap();

    Ok(())
}
