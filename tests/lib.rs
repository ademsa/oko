use minigreplib::{count, find, write_count_results};

use owo_colors::AnsiColors;
use std::io::BufReader;

#[test]
fn test_find() {
    let expected: Vec<(u32, Vec<(String, Option<String>)>)> = vec![(
        0,
        vec![
            ("Hello ".to_string(), Some("my".to_string())),
            (" friend!".to_string(), None),
        ],
    )];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find(&mut reader, "my", &false).unwrap();

    assert_eq!(expected, results);
}

#[test]
fn test_find_case_insensitive() {
    let expected: Vec<(u32, Vec<(String, Option<String>)>)> = vec![
        (
            0,
            vec![
                ("Hello ".to_string(), Some("my".to_string())),
                (" friend!".to_string(), None),
            ],
        ),
        (
            3,
            vec![
                ("".to_string(), Some("My".to_string())),
                (" name is Jack.".to_string(), None),
            ],
        ),
    ];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find(&mut reader, "my", &true).unwrap();

    assert_eq!(expected, results);
}

#[test]
fn test_find_regex() {
    let expected: Vec<(u32, Vec<(String, Option<String>)>)> = vec![(
        0,
        vec![
            ("Hello ".to_string(), Some("my".to_string())),
            (" friend!".to_string(), None),
        ],
    )];

    let data = b"Hello my friend!\nHow are you doing?\nNice to meet you!\nMy name is Jack.";
    let mut reader = BufReader::new(&data[..]);

    let results = find(&mut reader, "\\bmy\\b", &false).unwrap();

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

    let results = count(&mut reader, r"\byou\b", &true).unwrap();

    assert_eq!(results, expected);
}

#[test]
fn test_write_count_results() {
    let mut writer = Vec::new();

    let results = "1".to_string();

    write_count_results(results, AnsiColors::Green, &mut writer);

    assert_eq!(writer, "\u{1b}[32m1\u{1b}[39m\n".as_bytes());
}
