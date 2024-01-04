# minigrep

Find term in stdin or file content

### Running and Development

Build
```bash
cargo build
```

Run example
```bash
./target/debug/minigrep my content.txt
```

Run using pipe
```bash
cat content.txt | ./target/debug/minigrep my
```

Run tests
```bash
make test
```
