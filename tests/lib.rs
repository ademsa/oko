use minigreplib::{count, count_regex, find, find_regex, write_results};
use owo_colors::AnsiColors;

use std::io::BufReader;

#[test]
fn test_find() {
    let expected = vec!["1: Hello \u{1b}[32mmy\u{1b}[39m friend!".to_string()];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find(&mut reader, "my", &false, AnsiColors::Green).unwrap();

    assert_eq!(expected, results);
}

#[test]
fn test_find_case_insensitive() {
    let expected = vec![
        "1: hello \u{1b}[32mmy\u{1b}[39m friend!".to_string(),
        "4: \u{1b}[32mmy\u{1b}[39m name is jack.".to_string(),
    ];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find(&mut reader, "my", &true, AnsiColors::Green).unwrap();

    assert_eq!(expected, results);
}

#[test]
fn test_find_regex() {
    let expected = vec![
        "2: \u{1b}[32myou\u{1b}[39m".to_string(),
        "3: \u{1b}[32myou\u{1b}[39m".to_string(),
    ];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find_regex(&mut reader, "\\byou\\b", AnsiColors::Green).unwrap();

    assert_eq!(expected, results);
}

#[test]
fn test_count() {
    let expected = 1;

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = count(&mut reader, "my", &false).unwrap();

    assert_eq!(results, expected);
}

#[test]
fn test_count_case_insensitive() {
    let expected = 2;

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = count(&mut reader, "my", &true).unwrap();

    assert_eq!(results, expected);
}

#[test]
fn test_count_regex() {
    let expected = 2;

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = count_regex(&mut reader, r"\byou\b").unwrap();

    assert_eq!(results, expected);
}

#[test]
fn test_write_results() {
    let mut writer = Vec::new();

    let results = vec!["test".to_string()];

    write_results(results, &mut writer);

    assert_eq!(writer, "test\n".as_bytes());
}
