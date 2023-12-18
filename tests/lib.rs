use minigrep::find_matches;
use owo_colors::AnsiColors;

#[test]
fn find_match() {
    let mut result = Vec::new();
    find_matches(AnsiColors::Green, "hello\nmy\nfriend", "my", &mut result);
    assert_eq!(result, "2: \u{1b}[32mmy\u{1b}[39m\n".as_bytes());
}
