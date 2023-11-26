use minigrep::find_matches;

#[test]
fn find_match() {
    let mut result = Vec::new();
    find_matches("hello\nmy\nfriend", "my", &mut result);
    assert_eq!(result, b"2: my\n");
}
