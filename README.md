<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://github.com/ademsa/oko/raw/main/assets/img/oko-logo-darkmode.png" />
        <img alt="OKO logo" src="https://github.com/ademsa/oko/raw/main/assets/img/oko-logo.png" width="50%" />
    </picture>
</div>

<br/>

<div  align="center">
    CLI to Search, Count, Transform and Output Content
</div>

<br/>

<div align="center">

![Language](https://img.shields.io/badge/language-rust-red)
![License](https://img.shields.io/github/license/ademsa/oko)
![Release](https://img.shields.io/github/v/release/ademsa/oko)

</div>

### Features

- Search content in files or from stdin
- Count occurrences
- Regex search
- Exact match or ignore case search/count
- Output results in plain or json format
- Save results to console or file
- Color output
- Line number output
- Etc.

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
