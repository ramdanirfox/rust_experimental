## Rust Experimental
Initiative to list musl compatible library (the one that declared written in pute rust) for cross compile compatibility

## How
```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --release --target=x86_64-unknown-linux-musl
```
## Currently Tested
- [umya_spreadsheet](https://github.com/mathnya/umya-spreadsheet) (replacement for [xlsxwriter](https://docs.rs/xlsxwriter/latest/xlsxwriter/), able to read spreadsheet too!)