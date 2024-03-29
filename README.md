```
_____________ ________ 
__  __ \__  //_/_  __ \
_  / / /_  ,<  _ / / /
/ /_/ /_  /| | / /_/ /
\____/ /_/ |_| \____/
```

OKO: Search, Count, Transform and Output Content

### Running and Development

Build

```bash
make build
```

Run example

```bash
./target/debug/oko here -i ./examples/content.txt
```

Run using pipe

```bash
cat ./examples/content.txt | ./target/debug/oko here
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
- [x] Search pattern in content from files
- [x] Search pattern in content from stdin
- [x] Case-insensitive search
- [x] Regex search
- [x] Count mode
- [x] Add log level arg
- [x] Split output from search
- [x] Write output to file
- [x] Improved help text in CLI
- [x] Alternative outputs like json
- [x] Structured results
- [x] Arg to control if index is sent to output
- [x] Test Context (setup, teardown)
- [x] Improved test suite