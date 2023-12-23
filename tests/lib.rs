use minigrep::find_matches;
use owo_colors::AnsiColors;

use anyhow::Result;
use std::io::BufReader;

#[test]
fn find_match() -> Result<()> {
    let mut writer = Vec::new();

    let data = b"hello\nmy\nfriend";
    let mut reader = BufReader::new(&data[..]);

    find_matches(AnsiColors::Green, "my", &mut reader, &mut writer)?;

    assert_eq!(writer, "2: \u{1b}[32mmy\u{1b}[39m\n".as_bytes());

    Ok(())
}
