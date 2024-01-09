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
