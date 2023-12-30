use minigrep::find_matches;
use owo_colors::AnsiColors;

use anyhow::Result;
use std::io::BufReader;

#[test]
fn test_find_matches() -> Result<()> {
    let mut writer = Vec::new();

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is John.";
    let mut reader = BufReader::new(&data[..]);

    find_matches(
        AnsiColors::Green,
        "my",
        &false,
        &false,
        &mut reader,
        &mut writer,
    )?;

    assert_eq!(
        writer,
        "1: Hello \u{1b}[32mmy\u{1b}[39m friend!\n".as_bytes()
    );

    Ok(())
}

#[test]
fn test_find_matches_case_insensitive() -> Result<()> {
    let mut writer = Vec::new();

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is John.";
    let mut reader = BufReader::new(&data[..]);

    find_matches(
        AnsiColors::Green,
        "my",
        &false,
        &true,
        &mut reader,
        &mut writer,
    )?;

    assert_eq!(
        writer,
        "1: hello \u{1b}[32mmy\u{1b}[39m friend!\n4: \u{1b}[32mmy\u{1b}[39m name is john.\n"
            .as_bytes()
    );

    Ok(())
}

#[test]
fn test_count_matches() -> Result<()> {
    let mut writer = Vec::new();

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is John.";
    let mut reader = BufReader::new(&data[..]);

    find_matches(
        AnsiColors::Green,
        "my",
        &true,
        &false,
        &mut reader,
        &mut writer,
    )?;

    assert_eq!(writer, "\u{1b}[32m1\u{1b}[39m\n".as_bytes());

    Ok(())
}
