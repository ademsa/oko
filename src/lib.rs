//! # MinigrepLib
//!
//! Library for finding pattern in stdin or file content
use std::io::{BufRead, Result, Write};

use owo_colors::{AnsiColors, OwoColorize};
use regex::RegexBuilder;

/// Find a phrase or pattern
pub fn find<R: BufRead>(
    reader: &mut R,
    pattern: &str,
    ignore_case: &bool,
) -> Result<Vec<(u32, Vec<(String, Option<String>)>)>> {
    let target_pattern = RegexBuilder::new(&pattern)
        .case_insensitive(*ignore_case)
        .build();

    let mut results: Vec<(u32, Vec<(String, Option<String>)>)> = vec![];
    for (idx, line) in reader.by_ref().lines().enumerate() {
        let content = line?;
        let mut result: Vec<(String, Option<String>)> = vec![];
        let mut prev_m_end_idx = 0;
        for m in target_pattern.as_ref().unwrap().find_iter(content.as_str()) {
            // Write content before and match
            result.push((
                content[prev_m_end_idx..m.start()].to_string(),
                Some(m.as_str().to_string()),
            ));

            // Set current start index
            prev_m_end_idx = m.end();
        }

        if prev_m_end_idx != 0 {
            // Write remaining content
            result.push((content[prev_m_end_idx..].to_string(), None));

            // Store results for line
            results.push((idx as u32, result));
        }
    }

    Ok(results)
}

/// Count a phrase or pattern
pub fn count<R: BufRead>(reader: &mut R, pattern: &str, ignore_case: &bool) -> Result<u32> {
    let target_pattern = RegexBuilder::new(&pattern)
        .case_insensitive(*ignore_case)
        .build();

    let mut results = 0;
    for line in reader.by_ref().lines() {
        results += target_pattern
            .as_ref()
            .unwrap()
            .find_iter(line?.as_str())
            .count() as u32;
    }

    Ok(results)
}

/// Write find results
pub fn write_find_results(
    results: Vec<(u32, Vec<(String, Option<String>)>)>,
    color: AnsiColors,
    mut writer: impl Write,
) {
    for r in results.iter() {
        let (m_idx, m) = r;
        write!(writer, "{}: ", m_idx.to_string()).unwrap();
        for m_slice in m.iter() {
            let (content_before_match, m_pattern) = m_slice;
            if m_pattern.is_none() {
                write!(writer, "{}", content_before_match).unwrap();
            } else {
                write!(
                    writer,
                    "{}{}",
                    content_before_match,
                    m_pattern.as_ref().unwrap().as_str().color(color)
                )
                .unwrap();
            }
        }

        write!(writer, "\n").unwrap()
    }
}

///Write count results
pub fn write_count_results(results: String, color: AnsiColors, mut writer: impl Write) {
    writeln!(writer, "{}", results.color(color)).unwrap();
}
