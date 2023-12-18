use owo_colors::{AnsiColors, OwoColorize};
use std::io::Write;

pub fn find_matches(color: AnsiColors, content: &str, pattern: &str, mut writer: impl Write) {
    for (idx, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            // Write match and apply color
            writeln!(writer, "{}: {}", idx + 1, line.color(color)).unwrap();
        }
    }
}
