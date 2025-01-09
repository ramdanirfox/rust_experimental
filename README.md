## Rust Experimental
Initiative to list musl compatible library ( mostly the one that declared written in pure rust) for cross compile compatibility

## How
```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --release --target=x86_64-unknown-linux-musl
```
## Currently Tested
- [umya_spreadsheet](https://github.com/mathnya/umya-spreadsheet) (replacement for [xlsxwriter](https://docs.rs/xlsxwriter/latest/xlsxwriter/), able to read spreadsheet too!)

## Notes
- reqwest: configuring TLS to work with musl is a pain, just disabled features entirely.

## Caveats
- reqwest: No support for accessing https resource