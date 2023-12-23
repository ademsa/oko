use owo_colors::{AnsiColors, OwoColorize};

use std::io::{BufRead, Result, Write};

pub fn find_matches<R: BufRead>(
    color: AnsiColors,
    pattern: &str,
    reader: &mut R,
    mut writer: impl Write,
) -> Result<()> {
    for (idx, line) in reader.by_ref().lines().enumerate() {
        let content = line?;
        if content.contains(&pattern) {
            // Write match and apply color
            writeln!(writer, "{}: {}", idx + 1, content.color(color)).unwrap();
        }
    }
    Ok(())
}
