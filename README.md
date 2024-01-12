# minigrep

Find pattern in stdin or file content

### Running and Development

Build
```bash
cargo build
```

Run example
```bash
./target/debug/minigrep here content.txt
```

Run using pipe
```bash
cat content.txt | ./target/debug/minigrep here
```

Run tests
```bash
make test
```

### Progress

- [x] App Config
- [x] Lint tooling
- [x] Test tooling
- [x] Coverage tooling
- [x] Docs tooling
- [x] Colors
- [x] Find pattern in content from files
- [x] Find pattern in content from stdin
- [x] Case-insensitive search
- [x] Regex search
- [x] Count mode
- [x] Add log level arg
- [ ] Split output from search
- [ ] Alternative outputs like json
- [ ] Arg to control if index is sent to output
- [ ] Write output to file
