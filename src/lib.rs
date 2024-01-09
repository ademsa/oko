//! # MinigrepLib
//!
//! Library for finding pattern in stdin or file content
use owo_colors::{AnsiColors, OwoColorize};

use regex::Regex;
use std::io::{BufRead, Result, Write};

/// Find pattern
pub fn find<R: BufRead>(
    reader: &mut R,
    pattern: &str,
    ignore_case: &bool,
    color: AnsiColors,
) -> Result<Vec<String>> {
    let mut target_pattern = pattern.to_lowercase();
    if !*ignore_case {
        target_pattern = pattern.to_string();
    }

    let mut results: Vec<String> = vec![];
    for (idx, line) in reader.by_ref().lines().enumerate() {
        let mut content = line?;
        if *ignore_case {
            content = content.to_lowercase();
        }
        let matches = content.matches(&target_pattern);
        if matches.count() > 0 {
            let mut result = String::new();

            // Push line number
            result.push_str(format!("{}: ", idx + 1).as_str().as_ref());

            let mut prev_m_start_idx = 0;
            let mut prev_m_end_idx = 0;
            for (m_idx, m) in content.match_indices(&target_pattern) {
                // Push content before match
                result.push_str(
                    format!("{}", &content[prev_m_start_idx..m_idx])
                        .as_str()
                        .as_ref(),
                );

                // Push match
                result.push_str(format!("{}", m.color(color)).as_str().as_ref());

                // Set indexes
                prev_m_start_idx = m_idx;
                prev_m_end_idx = prev_m_start_idx + m.len();
            }
            // Push remaining content
            result.push_str(format!("{}", &content[prev_m_end_idx..]).as_str().as_ref());

            results.push(result);
        }
    }

    Ok(results)
}

/// Find pattern with regex
pub fn find_regex<R: BufRead>(
    reader: &mut R,
    pattern: &str,
    color: AnsiColors,
) -> Result<Vec<String>> {
    let pattern_regex = Regex::new(&pattern).unwrap();

    let mut results: Vec<String> = vec![];
    let mut captures;
    for (idx, line) in reader.lines().enumerate() {
        let content = line.unwrap();
        captures = pattern_regex.captures(content.as_str());
        if !captures.is_none() {
            let mut result = String::new();

            // Push line number
            result.push_str(format!("{}: ", idx + 1).as_str().as_ref());

            for (_, c) in captures.unwrap().iter().enumerate() {
                // Push match
                result.push_str(format!("{}", c.unwrap().as_str().color(color)).as_ref());
            }
            
            results.push(result);
        }
    }

    Ok(results)
}

/// Count pattern
pub fn count<R: BufRead>(reader: &mut R, pattern: &str, ignore_case: &bool) -> Result<u32> {
    let mut results_counter = 0;

    let mut target_pattern = pattern.to_lowercase();
    if !*ignore_case {
        target_pattern = pattern.to_string();
    }

    for line in reader.by_ref().lines() {
        let mut content = line?;
        if *ignore_case {
            content = content.to_lowercase();
        }
        let matches = content.matches(&target_pattern);
        if matches.count() > 0 {
            results_counter += content.match_indices(&target_pattern).count() as u32;
        }
    }

    Ok(results_counter)
}

/// Count pattern with regex
pub fn count_regex<R: BufRead>(reader: &mut R, pattern: &str) -> Result<u32> {
    let pattern_regex = Regex::new(&pattern).unwrap();

    let mut results_counter = 0;
    let mut captures;
    for line in reader.lines() {
        let content = line.unwrap();
        captures = pattern_regex.captures(content.as_str());
        if !captures.is_none() {
            results_counter += captures.unwrap().len() as u32
        }
    }

    Ok(results_counter)
}

/// Write results
pub fn write_results(results: Vec<String>, mut writer: impl Write) {
    writeln!(writer, "{}", results.join("\n")).unwrap();
}
