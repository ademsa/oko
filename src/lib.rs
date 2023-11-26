use std::io::Write;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for (idx, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            writeln!(writer, "{}: {}", idx + 1, line).unwrap();
        }
    }
}
