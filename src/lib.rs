//! # MinigrepLib
//!
//! Library for finding content in file
use owo_colors::{AnsiColors, OwoColorize};

use std::io::{BufRead, Result, Write};

/// Find matches
pub fn find_matches<R: BufRead>(
    color: AnsiColors,
    pattern: &str,
    count: &bool,
    ignore_case: &bool,
    reader: &mut R,
    mut writer: impl Write,
) -> Result<()> {
    let mut counter = 0;
    let mut term = pattern.to_lowercase();
    if !*ignore_case {
        term = pattern.to_string();
    }
    for (idx, line) in reader.by_ref().lines().enumerate() {
        let mut content = line?;
        if *ignore_case {
            content = content.to_lowercase();
        }
        let matches = content.matches(&term);
        if matches.count() > 0 {
            if !*count {
                // Write line number
                write!(writer, "{}: ", idx + 1).unwrap();
            }

            let mut prev_m_start_idx = 0;
            let mut prev_m_end_idx = 0;
            for (m_idx, m) in content.match_indices(&term) {
                if *count {
                    counter += 1;
                } else {
                    // Write content before match
                    write!(writer, "{}", &content[prev_m_start_idx..m_idx]).unwrap();

                    // Write match
                    write!(writer, "{}", m.color(color)).unwrap();

                    // Set indexes
                    prev_m_start_idx = m_idx;
                    prev_m_end_idx = prev_m_start_idx + m.len();
                }
            }
            if !*count {
                // Write remaining content
                write!(writer, "{}", &content[prev_m_end_idx..]).unwrap();

                // Wrap up this line
                writeln!(writer, "").unwrap();
            }
        }
    }
    if *count {
        // Write counter
        writeln!(writer, "{}", counter.color(color)).unwrap();
    }
    Ok(())
}
